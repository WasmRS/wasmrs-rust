#![allow(missing_debug_implementations)]
use crate::{Frame, PayloadError, RSocket};
use wasmrs_frames::{ErrorCode, FrameFlags, RSocketFlags};
use wasmrs_runtime::{self as runtime, unbounded_channel, Entry, SafeMap, UnboundedReceiver, UnboundedSender};
use wasmrs_rx::*;
mod buffer;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use futures::stream::{AbortHandle, Abortable};
use futures::{StreamExt, TryFutureExt};
mod responder;

pub use self::buffer::BufferState;
use self::responder::Responder;
use crate::{Error, Payload};

pub enum Handler {
  ReqRR(tokio::sync::oneshot::Sender<Result<Payload, PayloadError>>),
  ReqRS(Flux<Payload, PayloadError>),
  ReqRC(Flux<Payload, PayloadError>),
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Specify the socket side (only used for debugging).
pub enum SocketSide {
  /// A guest-side socket.
  Guest,
  /// A host-side socket.
  Host,
}

impl std::fmt::Display for SocketSide {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      SocketSide::Guest => "guest",
      SocketSide::Host => "host",
    })
  }
}

#[derive()]
#[must_use]
/// A socket that can be used to communicate between a host & guest via the wasmRS protocol.
pub struct WasmSocket {
  side: SocketSide,
  pub(super) handlers: Arc<SafeMap<u32, Handler>>,
  abort_handles: Arc<SafeMap<u32, AbortHandle>>,
  channels: Arc<SafeMap<u32, UnboundedSender<u32>>>,
  pub(super) stream_index: AtomicU32,
  tx: UnboundedSender<Frame>,
  rx: Option<UnboundedReceiver<Frame>>,
  responder: Responder,
}

impl std::fmt::Debug for WasmSocket {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ModuleState")
      .field("# pending streams", &self.handlers.len())
      .field("stream_index", &self.stream_index)
      .finish()
  }
}

impl WasmSocket {
  /// Create a new [WasmSocket] with the passed implementation of [RSocket].
  pub fn new(rsocket: impl RSocket + 'static, side: SocketSide) -> WasmSocket {
    let first_stream_id = match side {
      SocketSide::Guest => 1,
      SocketSide::Host => 2,
    };

    let (snd_tx, snd_rx) = unbounded_channel::<Frame>();
    let streams = Arc::new(Default::default());
    let abort_handles = Arc::new(Default::default());
    let channels = Arc::new(Default::default());

    WasmSocket {
      side,
      stream_index: AtomicU32::new(first_stream_id),
      tx: snd_tx,
      rx: Some(snd_rx),
      handlers: streams,
      abort_handles,
      channels,
      responder: Responder::new(Box::new(rsocket)),
    }
  }

  /// Take the receiver for this [WasmSocket].
  pub fn take_rx(&mut self) -> Result<UnboundedReceiver<Frame>, Error> {
    self.rx.take().ok_or(crate::Error::ReceiverAlreadyGone)
  }

  pub(crate) fn next_stream_id(&self) -> u32 {
    self.stream_index.fetch_add(2, Ordering::SeqCst)
  }

  /// Register a handler for a stream.
  pub fn register_handler(&self, stream_id: u32, handler: Handler) {
    self.handlers.insert(stream_id, handler);
  }

  /// Register a channel/stream for a stream_id.
  pub fn register_channel(&self, stream_id: u32) -> UnboundedReceiver<u32> {
    let (tx, rx) = unbounded_channel();
    self.channels.insert(stream_id, tx);
    rx
  }

  /// Send a frame.
  pub fn send(&self, frame: Frame) {
    send(&self.tx, self.side, frame);
  }

  /// Process a frame.
  pub fn process_once(&self, frame: Frame) -> Result<(), Error> {
    #[cfg(feature = "record-frames")]
    crate::record::write_incoming_record(self.side, frame.clone());
    let stream_id = frame.stream_id();
    trace!(stream_id, side = %self.side, kind = %frame.frame_type(), "process_once");
    let flag = frame.get_flag();
    match frame {
      Frame::RequestFnF(f) => {
        let input: Payload = f.into();
        self.on_request_fnf(stream_id, input);
      }
      Frame::RequestResponse(f) => {
        let input: Payload = f.into();
        self.on_request_response(stream_id, input);
      }
      Frame::RequestStream(f) => {
        let input: Payload = f.into();
        self.on_request_stream(stream_id, input);
      }
      Frame::RequestChannel(f) => {
        let input: Payload = f.into();
        self.on_request_channel(stream_id, input);
      }
      Frame::PayloadFrame(f) => {
        let input: Payload = f.into();
        self.on_payload(stream_id, flag, input);
      }
      Frame::Cancel(_) => {
        self.on_cancel(stream_id, flag);
      }
      Frame::ErrorFrame(f) => {
        self.on_error(
          stream_id,
          flag,
          f.code,
          if f.data.is_empty() {
            "Error frame with no data".to_owned()
          } else {
            f.data
          },
        );
      }
      Frame::RequestN(f) => {
        self.on_request_n(stream_id, f.n);
      }
    }

    Ok(())
  }

  fn on_request_response(&self, sid: u32, input: Payload) {
    trace!(
        sid,
        side = %self.side,
        "on_request_response"
    );
    let responder = self.responder.clone();
    let tx = self.tx.clone();
    let result = responder.request_response(input);
    let side = self.side;

    runtime::spawn(async move {
      match result.await {
        Ok(res) => {
          send_payload(&tx, sid, side, res, Frame::FLAG_NEXT | Frame::FLAG_COMPLETE);
        }
        Err(e) => send_app_error(&tx, sid, side, e.to_string()),
      };
    });
  }

  fn on_request_stream(&self, sid: u32, input: Payload) {
    trace!(sid, side = %self.side, "on_request_stream");
    let responder = self.responder.clone();
    let tx = self.tx.clone();
    let abort_handles = self.abort_handles.clone();
    let side = self.side;

    runtime::spawn(async move {
      let (abort_handle, abort_registration) = AbortHandle::new_pair();
      abort_handles.insert(sid, abort_handle);
      let mut payloads = Abortable::new(responder.request_stream(input), abort_registration);

      while let Some(next) = payloads.next().await {
        match next {
          Ok(it) => send_payload(&tx, sid, side, it, Frame::FLAG_NEXT),
          Err(e) => send_app_error(&tx, sid, side, e.to_string()),
        };
      }
      abort_handles.remove(&sid);
      send_complete(&tx, sid, side, Frame::FLAG_COMPLETE);
    });
  }

  fn on_request_channel(&self, sid: u32, first: Payload) {
    trace!(sid, side = %self.side, "on_request_channel");
    let responder = self.responder.clone();

    let tx = self.tx.clone();
    let (handler_tx, handler_rx) = Flux::new_channels();

    handler_tx.send(first).unwrap();
    self.register_handler(sid, Handler::ReqRC(handler_tx));
    let abort_handles = self.abort_handles.clone();
    let side = self.side;

    runtime::spawn(async move {
      let outputs = responder.request_channel(handler_rx);
      let (abort_handle, abort_registration) = AbortHandle::new_pair();
      abort_handles.insert(sid, abort_handle);
      let mut outputs = Abortable::new(outputs, abort_registration);

      // TODO: support custom RequestN.
      let request_n = Frame::new_request_n(sid, Frame::REQUEST_MAX, 0);

      send(&tx, side, request_n);

      while let Some(next) = outputs.next().await {
        let sending = match next {
          Ok(payload) => Frame::new_payload(sid, payload, Frame::FLAG_NEXT),
          Err(e) => Frame::new_error(sid, ErrorCode::ApplicationError.into(), e.to_string()),
        };
        send(&tx, side, sending);
      }
      abort_handles.remove(&sid);
      let complete = Frame::new_payload(sid, Payload::empty(), Frame::FLAG_COMPLETE);
      send(&tx, side, complete);
    });
  }

  fn on_request_fnf(&self, sid: u32, input: Payload) {
    trace!(sid, side = %self.side, "on_request_fnf");

    let responder = self.responder.clone();
    let tx = self.tx.clone();
    let result = responder.fire_and_forget(input);

    let side = self.side;
    runtime::spawn(async move {
      if let Err(e) = result.await {
        send_app_error(&tx, sid, side, e.msg);
      }
    });
  }

  fn on_request_n(&self, sid: u32, n: u32) {
    trace!(sid, side = %self.side, "on_request_n");
    let tx = self.tx.clone();
    if n == 0 {
      send_app_error(&tx, sid, self.side, "Invalid RequestN (n=0)");
      return;
    }
    #[allow(clippy::option_if_let_else)]
    match self.channels.cloned(&sid) {
      Some(reqn_tx) => {
        if reqn_tx.send(n).is_err() {
          send_app_error(&tx, sid, self.side, "RequestN channel closed");
        };
      }
      None => {
        send_app_error(&tx, sid, self.side, "RequestN called for missing Stream ID");
      }
    }
  }

  fn on_payload(&self, sid: u32, flag: FrameFlags, input: Payload) {
    trace!(sid, side = %self.side, "on_payload");
    let tx = self.tx.clone();
    match self.handlers.entry(sid) {
      Entry::Occupied(o) => match o.get() {
        Handler::ReqRR(_) => match o.remove() {
          Handler::ReqRR(sender) => {
            if flag.flag_next() && sender.send(Ok(input)).is_err() {
              error!(sid, side = %self.side, "response successful payload for REQUEST_RESPONSE failed");
            }
          }
          _ => unreachable!(),
        },
        Handler::ReqRS(sender) => {
          if flag.flag_next() {
            if sender.is_closed() {
              send_cancel(&tx, sid, self.side);
            } else if let Err(_e) = sender.send(input) {
              error!(sid, side = %self.side, "response successful payload for REQUEST_STREAM failed");
              send_cancel(&tx, sid, self.side);
            }
          }
          if flag.flag_complete() {
            trace!(sid, "removing stream");
            o.remove();
          }
        }
        Handler::ReqRC(sender) => {
          if flag.flag_next() {
            if sender.is_closed() {
              send_cancel(&tx, sid, self.side);
            } else if (sender.send(input)).is_err() {
              error!(sid, side = %self.side, "response successful payload for REQUEST_CHANNEL failed");
              send_cancel(&tx, sid, self.side);
            }
          }
          if flag.flag_complete() {
            trace!(sid, "removing channel");
            o.remove();
          }
        }
      },
      Entry::Vacant(_) => {
        warn!(sid, side = %self.side, "response successful payload for REQUEST_CHANNEL failed");
      }
    }
  }

  fn on_cancel(&self, sid: u32, _flag: FrameFlags) {
    trace!(sid, side = %self.side, "on_cancel");
    if let Some(handler) = self.handlers.remove(&sid) {
      let e = PayloadError::new(ErrorCode::Canceled.into(), "Request cancelled");
      match handler {
        Handler::ReqRR(sender) => {
          sender.send(Err(e)).unwrap();
        }
        Handler::ReqRS(_) => {
          // stream cancelled. Take no action besides removing the handler.
        }
        Handler::ReqRC(_) => {
          // stream cancelled. Take no action besides removing the handler.
        }
      }
    }
  }

  fn on_error(&self, sid: u32, _flag: FrameFlags, code: u32, message: String) {
    trace!(sid, code, message, side = %self.side, "on_error");
    if let Some(handler) = self.handlers.remove(&sid) {
      let e = PayloadError::new(code, message);
      match handler {
        Handler::ReqRR(sender) => {
          sender.send(Err(e)).unwrap();
        }
        Handler::ReqRS(sender) => {
          sender.error(e).unwrap();
        }
        Handler::ReqRC(sender) => {
          sender.error(e).unwrap();
        }
      }
    }
  }
}

impl RSocket for WasmSocket {
  fn fire_and_forget(&self, payload: Payload) -> Mono<(), PayloadError> {
    let sid = self.next_stream_id();
    trace!(sid, side = %self.side, "request_response");

    let frame = Frame::new_request_fnf(sid, payload, 0);

    send(&self.tx, self.side, frame);

    Mono::new_success(())
  }

  fn request_response(&self, payload: Payload) -> Mono<Payload, PayloadError> {
    let sid = self.next_stream_id();
    trace!(sid, side = %self.side, "request_response");

    let (tx, rx) = tokio::sync::oneshot::channel();

    self.register_handler(sid, Handler::ReqRR(tx));
    let frame = Frame::new_request_response(sid, payload, 0);

    send(&self.tx, self.side, frame);
    let fut = rx.map_err(|_e| PayloadError::application_error("Request-response channel failed"));

    Mono::<Payload, PayloadError>::from_future(async move { fut.await? })
  }

  fn request_stream(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
    let sid = self.next_stream_id();
    trace!(sid, side = %self.side, "request_stream");

    let (flux, output) = Flux::new_channels();

    self.register_handler(sid, Handler::ReqRS(flux));

    let frame = Frame::new_request_stream(sid, payload, 0);

    send(&self.tx, self.side, frame);

    output
  }

  fn request_channel(&self, mut stream: FluxReceiver<Payload, PayloadError>) -> FluxReceiver<Payload, PayloadError> {
    let sid = self.next_stream_id();
    trace!(sid, side = %self.side, "request_channel");

    let (flux, output) = Flux::new_channels();

    self.register_handler(sid, Handler::ReqRC(flux));
    let mut reqn_rx = self.register_channel(sid);
    let tx = self.tx.clone();
    let channels = self.channels.clone();

    let side = self.side;
    runtime::spawn(async move {
      let mut first = true;
      let mut n = 1;
      while let Some(next) = stream.next().await {
        n -= 1;
        match next {
          Ok(payload) => {
            if first {
              first = false;
              send_channel(&tx, sid, side, payload, Frame::FLAG_NEXT);
            } else {
              send_payload(&tx, sid, side, payload, Frame::FLAG_NEXT);
            }
          }
          Err(_e) => {
            send_app_error(&tx, sid, side, "REQUEST_CHANNEL failed");
          }
        }
        // If we've exhausted our requested n, wait for the next RequestN frame
        if n == 0 {
          if let Some(new_n) = reqn_rx.recv().await {
            n = new_n;
          } else {
            break;
          }
        }
      }
      channels.remove(&sid);
      send_complete(&tx, sid, side, Frame::FLAG_COMPLETE);
    });

    output
  }
}

fn send(tx: &UnboundedSender<Frame>, _side: SocketSide, frame: Frame) {
  trace!("sending frame to socket writer: {:?}", frame);
  #[cfg(feature = "record-frames")]
  crate::record::write_outgoing_record(_side, frame.clone());

  tx.send(frame).unwrap();
}

fn send_payload(tx: &UnboundedSender<Frame>, sid: u32, side: SocketSide, payload: Payload, flag: FrameFlags) {
  send(tx, side, Frame::new_payload(sid, payload, flag));
}

fn send_channel(tx: &UnboundedSender<Frame>, sid: u32, side: SocketSide, payload: Payload, flag: FrameFlags) {
  send(
    tx,
    side,
    Frame::new_request_channel(sid, payload, flag, Frame::REQUEST_MAX),
  );
}

fn send_cancel(tx: &UnboundedSender<Frame>, sid: u32, side: SocketSide) {
  send(tx, side, Frame::new_cancel(sid));
}

fn send_complete(tx: &UnboundedSender<Frame>, sid: u32, side: SocketSide, flag: FrameFlags) {
  send(tx, side, Frame::new_payload(sid, Payload::empty(), flag));
}

fn send_app_error(tx: &UnboundedSender<Frame>, sid: u32, side: SocketSide, msg: impl AsRef<str>) {
  let error = Frame::new_error(sid, ErrorCode::ApplicationError.into(), msg.as_ref());
  send(tx, side, error);
}

#[cfg(test)]
mod test {

  use anyhow::Result;
  use bytes::Bytes;

  use super::*;
  struct EchoRSocket;

  impl RSocket for EchoRSocket {
    fn fire_and_forget(&self, _payload: Payload) -> Mono<(), PayloadError> {
      /* no op */
      Mono::from_future(async { Ok(()) })
    }

    fn request_response(&self, payload: Payload) -> Mono<Payload, PayloadError> {
      info!("{:?}", payload);
      Mono::new_success(payload)
    }

    fn request_stream(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
      info!("{:?}", payload);
      let (tx, rx) = Flux::new_channels();
      tx.send(payload.clone()).unwrap();
      tx.send(payload).unwrap();
      tx.complete();
      rx
    }

    fn request_channel(&self, mut stream: FluxReceiver<Payload, PayloadError>) -> FluxReceiver<Payload, PayloadError> {
      let (tx, rx) = Flux::new_channels();
      runtime::spawn(async move {
        while let Some(next) = stream.next().await {
          tx.send_result(next).unwrap();
        }
        tx.complete();
      });
      rx
    }
  }

  fn make_echo() -> (Arc<WasmSocket>, Arc<WasmSocket>) {
    let mut guest = WasmSocket::new(EchoRSocket {}, SocketSide::Guest);
    let mut guest_frame_rx = guest.take_rx().unwrap();
    let mut host = WasmSocket::new(EchoRSocket {}, SocketSide::Host);
    let mut host_frame_rx = host.take_rx().unwrap();

    let guest = Arc::new(guest);
    let inner_guest = guest.clone();
    let host = Arc::new(host);
    let inner_host = host.clone();

    runtime::spawn(async move {
      while let Some(frame) = guest_frame_rx.recv().await {
        println!("GUEST >>> HOST: {:?}", frame);
        inner_host.process_once(frame).unwrap();
      }
    });
    runtime::spawn(async move {
      while let Some(frame) = host_frame_rx.recv().await {
        println!("HOST >>> GUEST: {:?}", frame);
        inner_guest.process_once(frame).unwrap();
      }
    });
    (guest, host)
  }

  #[test_log::test(tokio::test)]
  async fn test_fnf() -> Result<()> {
    let (guest, _host) = make_echo();

    let output = guest
      .fire_and_forget(Payload::new(Bytes::from_static(b""), Bytes::from_static(b"FNF")))
      .await;
    assert!(output.is_ok());

    Ok(())
  }

  #[test_log::test(tokio::test)]
  async fn test_reqres() -> Result<()> {
    let (guest, _host) = make_echo();

    let output = guest.request_response(Payload::new(Bytes::from_static(b""), Bytes::from_static(b"REQRES")));
    let once = output.await.unwrap();
    assert_eq!(once.data, Some(Bytes::from_static(b"REQRES")));
    Ok(())
  }

  #[test_log::test(tokio::test)]
  async fn test_reqstream() -> Result<()> {
    let (guest, _host) = make_echo();

    let mut output = guest.request_stream(Payload::new(Bytes::from_static(b""), Bytes::from_static(b"REQ_STR")));
    let once = output.next().await.unwrap().unwrap();
    assert_eq!(once.data, Some(Bytes::from_static(b"REQ_STR")));
    let once = output.next().await.unwrap().unwrap();
    assert_eq!(once.data, Some(Bytes::from_static(b"REQ_STR")));
    Ok(())
  }

  #[test_log::test(tokio::test)]
  async fn test_reqchannel() -> Result<()> {
    let (guest, _host) = make_echo();
    let (tx, rx) = Flux::new_channels();

    let mut output = guest.request_channel(rx);
    tx.send(Payload::new(
      Bytes::from_static(b""),
      Bytes::from_static(b"REQCHANNEL1"),
    ))
    .unwrap();
    tx.send(Payload::new(
      Bytes::from_static(b""),
      Bytes::from_static(b"REQCHANNEL2"),
    ))
    .unwrap();
    tx.complete();
    let once = output.next().await.unwrap().unwrap();
    assert_eq!(once.data, Some(Bytes::from_static(b"REQCHANNEL1")));
    let once = output.next().await.unwrap().unwrap();
    assert_eq!(once.data, Some(Bytes::from_static(b"REQCHANNEL2")));
    Ok(())
  }
}
