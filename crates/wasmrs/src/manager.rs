#![allow(missing_debug_implementations)]
use crate::flux::FluxChannel;
use crate::runtime::{self, channel, Entry, Receiver, SafeMap, Sender};
use crate::{ErrorCode, Frame, FrameFlags, PayloadError, RSocket};
mod buffer;

use futures::stream::{AbortHandle, Abortable};
use futures::StreamExt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::oneshot;
mod responder;
use bytes::Bytes;

use crate::{Counter, Error, Payload};

use self::buffer::BufferState;
use self::responder::Responder;

pub struct StreamState(Handler, ());
// pub type OutgoingStream = LocalSubject<'static, Vec<u8>, ()>;

pub type OptionalResult = Result<Option<Payload>, PayloadError>;
pub type StreamResult = Result<Payload, PayloadError>;

pub enum Handler {
    ReqRR(oneshot::Sender<OptionalResult>),
    ResRRn(Counter),
    ReqRS(FluxChannel<Payload, PayloadError>),
    ReqRC(FluxChannel<Payload, PayloadError>),
}

#[derive()]
#[must_use]
pub struct SocketManager {
    pub(super) streams: Arc<SafeMap<u32, Handler>>,
    abort_handles: Arc<SafeMap<u32, AbortHandle>>,

    host_buffer: BufferState,
    guest_buffer: BufferState,
    pub(super) stream_index: AtomicU32,
    tx: Sender<Frame>,
    rx: Option<Receiver<Frame>>,
    // _handler: TaskHandle,
    responder: Responder,
}

impl std::fmt::Debug for SocketManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleState")
            .field("# pending streams", &self.streams.len())
            .field("stream_index", &self.stream_index)
            .finish()
    }
}

impl SocketManager {
    pub fn new(rsocket: impl RSocket + 'static) -> SocketManager {
        let (snd_tx, snd_rx) = channel::<Frame>();
        let streams: Arc<SafeMap<u32, Handler>> = Arc::new(Default::default());
        let abort_handles: Arc<SafeMap<u32, AbortHandle>> = Arc::new(Default::default());

        // let handle = incoming_handler(snd_rx, Box::new(rsocket));

        SocketManager {
            stream_index: AtomicU32::new(1),
            tx: snd_tx,
            rx: Some(snd_rx),
            streams,
            abort_handles,
            // _handler: handle,
            host_buffer: Default::default(),
            guest_buffer: Default::default(),
            responder: Responder::new(Box::new(rsocket)),
        }
    }

    pub fn take_rx(&mut self) -> Result<Receiver<Frame>, Error> {
        self.rx.take().ok_or(Error::RxMissing)
    }

    pub fn host_buffer(&self) -> &BufferState {
        &self.host_buffer
    }

    pub fn guest_buffer(&self) -> &BufferState {
        &self.guest_buffer
    }

    pub fn new_stream(&self, handler: Handler) -> u32 {
        let id = self.stream_index.fetch_add(2, Ordering::SeqCst);
        self.streams.insert(id, handler);
        id
    }

    pub fn register_handler(&self, stream_id: u32, handler: Handler) {
        self.streams.insert(stream_id, handler);
    }

    pub fn take_stream(&self, stream_id: u32) -> Option<Handler> {
        self.streams.remove(&stream_id)
    }

    pub fn process_once(&self, frame: Frame) -> Result<(), Error> {
        let stream_id = frame.stream_id();
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
                self.on_request_stream(stream_id, flag, input);
            }
            Frame::RequestChannel(f) => {
                let input: Payload = f.into();
                self.on_request_channel(stream_id, flag, input);
            }
            Frame::PayloadFrame(f) => {
                let input: Payload = f.into();
                self.on_payload(stream_id, flag, input);
            }
            Frame::Cancel(_) => {
                self.on_cancel(stream_id, flag);
            }
            Frame::ErrorFrame(f) => {
                self.on_error(stream_id, flag, f.code, f.data);
            }
            Frame::RequestN(_) => {
                todo!();
            }
        }

        Ok(())
    }

    fn on_request_response(&self, stream_id: u32, input: Payload) {
        let responder = self.responder.clone();
        let mut tx = self.tx.clone();
        let counter = Counter::new(2);
        self.register_handler(stream_id, Handler::ResRRn(counter.clone()));
        let result = responder.request_response(input);

        runtime::spawn(async move {
            if counter.count_down() == 0 {
                // cancelled
                return;
            }

            match result.recv().await {
                Ok(Some(Ok(res))) => send_payload(
                    &mut tx,
                    stream_id,
                    res,
                    Frame::FLAG_NEXT | Frame::FLAG_COMPLETE,
                ),
                Ok(None) => send_complete(&mut tx, stream_id, Frame::FLAG_COMPLETE),
                Err(e) => send_error(
                    &mut tx,
                    Frame::new_error(stream_id, ErrorCode::ApplicationError.into(), e.to_string()),
                ),
                Ok(Some(Err(e))) => send_error(
                    &mut tx,
                    Frame::new_error(stream_id, ErrorCode::ApplicationError.into(), e.to_string()),
                ),
            };
        });
    }

    fn on_request_stream(&self, stream_id: u32, _flag: FrameFlags, input: Payload) {
        let responder = self.responder.clone();
        let mut tx = self.tx.clone();
        let abort_handles = self.abort_handles.clone();
        runtime::spawn(async move {
            let (abort_handle, abort_registration) = AbortHandle::new_pair();
            abort_handles.insert(stream_id, abort_handle);
            let mut payloads = Abortable::new(responder.request_stream(input), abort_registration);
            while let Some(next) = payloads.next().await {
                match next {
                    Ok(it) => send_payload(&mut tx, stream_id, it, Frame::FLAG_NEXT),
                    Err(e) => send_error(
                        &mut tx,
                        Frame::new_error(
                            stream_id,
                            ErrorCode::ApplicationError.into(),
                            e.to_string(),
                        ),
                    ),
                };
            }
            abort_handles.remove(&stream_id);
            send_complete(&mut tx, stream_id, Frame::FLAG_COMPLETE);
        });
    }

    fn on_request_channel(&self, _stream_id: u32, _flag: FrameFlags, _input: Payload) {
        todo!();
    }
    fn on_request_fnf(&self, _stream_id: u32, _input: Payload) {}
    fn on_payload(&self, stream_id: u32, flag: FrameFlags, input: Payload) {
        let mut tx = self.tx.clone();
        match self.streams.entry(stream_id) {
            Entry::Occupied(o) => {
                match o.get() {
                    Handler::ReqRR(_) => match o.remove() {
                        Handler::ReqRR(sender) => {
                            if flag & Frame::FLAG_NEXT != 0 {
                                if sender.send(Ok(Some(input))).is_err() {
                                    println!("response successful payload for REQUEST_RESPONSE failed: sid={}",stream_id);
                                }
                            } else if sender.send(Ok(None)).is_err() {
                                println!("response successful payload for REQUEST_RESPONSE failed: sid={}",stream_id);
                            }
                        }
                        _ => unreachable!(),
                    },
                    Handler::ResRRn(_c) => unreachable!(),
                    Handler::ReqRS(sender) => {
                        if flag & Frame::FLAG_NEXT != 0 {
                            if sender.is_closed() {
                                send_cancel(&mut tx, stream_id);
                            } else if let Err(_e) = sender.send(input) {
                                println!(
                                    "response successful payload for REQUEST_STREAM failed: sid={}",
                                    stream_id
                                );
                                send_cancel(&mut tx, stream_id);
                            }
                        }
                        if flag & Frame::FLAG_COMPLETE != 0 {
                            o.remove();
                        }
                    }
                    Handler::ReqRC(sender) => {
                        // TODO: support channel
                        if flag & Frame::FLAG_NEXT != 0 {
                            if sender.is_closed() {
                                send_cancel(&mut tx, stream_id);
                            } else if (sender.clone().send(input)).is_err() {
                                println!("response successful payload for REQUEST_CHANNEL failed: sid={}",stream_id);
                                send_cancel(&mut tx, stream_id);
                            }
                        }
                        if flag & Frame::FLAG_COMPLETE != 0 {
                            o.remove();
                        }
                    }
                }
            }
            Entry::Vacant(_) => println!("invalid payload id {}: no such request!", stream_id),
        }
    }
    fn on_cancel(&self, _stream_id: u32, _flag: FrameFlags) {}
    fn on_error(&self, _stream_id: u32, _flag: FrameFlags, _code: u32, _message: String) {}

    /// Invoked after a guest has completed its initialization.
    pub fn do_host_init(
        &self,
        guest_buff_ptr: u32,
        host_buff_ptr: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.host_buffer().update_start(host_buff_ptr);
        self.guest_buffer().update_start(guest_buff_ptr);
        Ok(())
    }

    /// Invoked when the guest module wishes to send a stream frame to the host.
    pub fn do_host_send(&self, frame_bytes: Bytes) -> Result<(), Box<dyn std::error::Error>> {
        let _result = match Frame::decode(frame_bytes) {
            Ok(frame) => self.process_once(frame),
            Err((stream_id, err)) => self
                .tx
                .send(Frame::new_error(stream_id, 0, err.to_string())),
        };
        Ok(())
    }

    /// Invoked when the guest module wants to write a message to the host's `stdout`
    pub fn do_console_log(&self, msg: &str) {
        println!("{}", msg);
    }
}

fn send_payload(tx: &mut Sender<Frame>, stream_id: u32, payload: Payload, flag: FrameFlags) {
    let sending = Frame::new_payload(stream_id, payload, flag);
    let _ = tx.send(sending);
}

fn send_cancel(tx: &mut Sender<Frame>, stream_id: u32) {
    let sending = Frame::new_cancel(stream_id);
    let _ = tx.send(sending);
}

fn send_complete(tx: &mut Sender<Frame>, stream_id: u32, flag: FrameFlags) {
    let sending = Frame::new_payload(stream_id, Payload::empty(), flag);
    let _ = tx.send(sending);
}

fn send_error(tx: &mut Sender<Frame>, error: Frame) {
    if let Err(e) = tx.send(error) {
        println!("respond REQUEST_RESPONSE failed: {}", e);
    }
}
