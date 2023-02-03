use std::cell::UnsafeCell;
use std::io::{Cursor, Write};
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use wasmrs::util::to_u24_bytes;
use wasmrs::SocketSide;
pub use wasmrs::{Frame, Metadata, OperationList, OperationType, Payload, PayloadError, RSocket};
pub use wasmrs_runtime::spawn;
use wasmrs_runtime::{exhaust_pool, UnboundedReceiver};
pub use wasmrs_rx::*;

/// An alias to [Box<dyn std::error::Error + Send + Sync + 'static>]
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
/// An alias for a [Vec<(String, String, Rc<T>)>]
pub type OperationMap<T> = Vec<(String, String, Rc<T>)>;
/// An alias for the function that creates the output for a task.
pub type ProcessFactory<I, O> = fn(I) -> Result<O, GenericError>;

/// An alias for [Mono<ParsedPayload, PayloadError>]
pub type IncomingMono = Mono<ParsedPayload, PayloadError>;
/// An alias for [Mono<Payload, PayloadError>]
pub type OutgoingMono = Mono<Payload, PayloadError>;
/// An alias for [FluxReceiver<ParsedPayload, PayloadError>]
pub type IncomingStream = FluxReceiver<ParsedPayload, PayloadError>;
/// An alias for [FluxReceiver<Payload, PayloadError>]
pub type OutgoingStream = FluxReceiver<Payload, PayloadError>;

pub use bytes::Bytes;
pub use futures_util::stream::select_all;
pub use futures_util::StreamExt;
pub use wasmrs_codec::messagepack::{deserialize, serialize};

use crate::error::Error;
use crate::imports::{_host_op_list, _host_wasmrs_init, _host_wasmrs_send};
use crate::server::WasmServer;

thread_local! {
  static GUEST_BUFFER: UnsafeCell<Vec<u8>> = UnsafeCell::new(Vec::new());
  static HOST_BUFFER: UnsafeCell<Vec<u8>> = UnsafeCell::new(Vec::new());
  static MAX_HOST_FRAME_SIZE: AtomicU32 = AtomicU32::new(128);
  pub(crate) static REQUEST_RESPONSE_HANDLERS: UnsafeCell<OperationMap<ProcessFactory<IncomingMono,OutgoingMono>>> = UnsafeCell::new(OperationMap::new());
  pub(crate) static REQUEST_STREAM_HANDLERS: UnsafeCell<OperationMap<ProcessFactory<IncomingMono,OutgoingStream>>> = UnsafeCell::new(OperationMap::new());
  pub(crate) static REQUEST_CHANNEL_HANDLERS: UnsafeCell<OperationMap<ProcessFactory<IncomingStream,OutgoingStream>>> = UnsafeCell::new(OperationMap::new());
  pub(crate) static REQUEST_FNF_HANDLERS: UnsafeCell<OperationMap<ProcessFactory<IncomingMono,()>>> = UnsafeCell::new(OperationMap::new());
  pub(crate) static OP_LIST: UnsafeCell<OperationList> = UnsafeCell::new(OperationList::default());
  pub(crate) static OP_LIST_BYTES: UnsafeCell<Vec<u8>> = UnsafeCell::new(Vec::new());
  static SOCKET: UnsafeCell<wasmrs::WasmSocket> = UnsafeCell::new(wasmrs::WasmSocket::new(WasmServer{}, SocketSide::Guest));
}

#[allow(missing_debug_implementations, missing_copy_implementations)]
#[derive(Default)]
/// The Host inside a WebAssembly module that implements [RSocket]
pub struct Host();

impl RSocket for Host {
  fn fire_and_forget(&self, payload: Payload) -> Mono<(), PayloadError> {
    SOCKET.with(|cell| {
      #[allow(unsafe_code)]
      let socket = unsafe { &mut *cell.get() };
      socket.fire_and_forget(payload)
    })
  }

  fn request_response(&self, payload: Payload) -> Mono<Payload, PayloadError> {
    SOCKET.with(|cell| {
      #[allow(unsafe_code)]
      let socket = unsafe { &mut *cell.get() };
      socket.request_response(payload)
    })
  }

  fn request_stream(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
    SOCKET.with(|cell| {
      #[allow(unsafe_code)]
      let socket = unsafe { &mut *cell.get() };
      socket.request_stream(payload)
    })
  }

  fn request_channel(&self, stream: FluxReceiver<Payload, PayloadError>) -> FluxReceiver<Payload, PayloadError> {
    SOCKET.with(|cell| {
      #[allow(unsafe_code)]
      let socket = unsafe { &mut *cell.get() };
      socket.request_channel(stream)
    })
  }
}

#[allow(missing_debug_implementations)]
#[derive(Debug)]
/// A [Payload] with pre-parsed [Metadata].
pub struct ParsedPayload {
  /// The parsed [Metadata].
  pub metadata: Metadata,
  /// The raw data bytes.
  pub data: Bytes,
}

impl TryFrom<Payload> for ParsedPayload {
  type Error = Error;

  fn try_from(value: Payload) -> Result<Self, Self::Error> {
    Ok(ParsedPayload {
      metadata: value.parse_metadata()?,
      data: value.data.unwrap_or_default(),
    })
  }
}

/// This is called as part of the module initialization for wasmRS.
pub fn init(guest_buffer_size: u32, host_buffer_size: u32, max_host_frame_len: u32) {
  tracing::trace!(
    "guest::init({}, {}, {}) called",
    guest_buffer_size,
    host_buffer_size,
    max_host_frame_len
  );

  let guest_ptr = GUEST_BUFFER.with(|cell| {
    #[allow(unsafe_code)]
    let buffer = unsafe { &mut *cell.get() };
    buffer.resize(guest_buffer_size as usize, 0);
    buffer.as_ptr()
  });
  let rx = SOCKET.with(|cell| {
    #[allow(unsafe_code)]
    let manager = unsafe { &mut *cell.get() };
    manager.take_rx().unwrap()
  });
  let host_ptr = HOST_BUFFER.with(|cell| {
    #[allow(unsafe_code)]
    let buffer = unsafe { &mut *cell.get() };
    buffer.resize(host_buffer_size as usize, 0);
    buffer.as_ptr()
  });
  MAX_HOST_FRAME_SIZE.with(|cell| cell.store(max_host_frame_len, Ordering::Relaxed));

  spawn_writer(rx);

  #[allow(unsafe_code)]
  unsafe {
    _host_wasmrs_init(guest_ptr as _, host_ptr as _);
  }
}

#[allow(unsafe_code)]
pub(crate) fn op_list_request() {
  let bytes = OP_LIST.with(|cell| unsafe { cell.get().as_ref().unwrap() }.encode());

  let (ptr, len) = OP_LIST_BYTES.with(|cell| {
    let buff = unsafe { &mut *cell.get() };
    *buff = bytes.to_vec();
    (buff.as_ptr(), buff.len())
  });

  unsafe {
    _host_op_list(ptr as _, len);
  }
}

#[allow(unsafe_code)]
pub(crate) fn send_frame(read_until: u32) {
  tracing::trace!(read_until, "__wasmrs_send() called");
  let read_result = read_frames(read_until);
  if read_result.is_err() {
    tracing::error!("could not read local buffer");
    send_error_frame(0, 0, "Could not read local buffer");
    return;
  }
  let bytes_list = read_result.unwrap();

  SOCKET.with(|cell| {
    let socket = unsafe { &mut *cell.get() };
    for bytes in bytes_list {
      match Frame::decode(bytes) {
        Ok(frame) => {
          let _ = socket.process_once(frame);
        }
        Err(_e) => {
          send_error_frame(0, 0, "Could not decode frame data");
          continue;
        }
      }
    }
  });

  exhaust_pool();
}

fn spawn_writer(mut rx: UnboundedReceiver<Frame>) {
  spawn(async move {
    while let Some(frame) = rx.recv().await {
      send_host_frame(vec![frame.encode()]);
    }
  });
}

fn read_frames(read_until: u32) -> Result<Vec<Bytes>, Error> {
  GUEST_BUFFER.with(|cell| {
    #[allow(unsafe_code)]
    let buff = unsafe { &mut *cell.get() };
    let mut buff = Cursor::new(buff);
    let mut frames = Vec::new();
    while buff.position() < read_until as _ {
      match wasmrs::util::read_frame(&mut buff) {
        Ok(bytes) => frames.push(bytes),
        Err(_e) => return Err(Error::BufferRead),
      }
    }
    Ok(frames)
  })
}

fn send_error_frame(stream_id: u32, code: u32, msg: impl AsRef<str>) {
  let err = Frame::new_error(stream_id, code, msg.as_ref());
  send_host_frame(vec![err.encode()]);
}

fn add_export(index: u32, kind: OperationType, namespace: impl AsRef<str>, operation: impl AsRef<str>) {
  OP_LIST.with(|op_list| {
    #[allow(unsafe_code)]
    let op_list = unsafe { &mut *op_list.get() };
    op_list.add_export(index, kind, namespace, operation);
  });
}

/// Add an imported wasmRS method for the module.
pub fn add_import(index: u32, kind: OperationType, namespace: impl AsRef<str>, operation: impl AsRef<str>) {
  OP_LIST.with(|op_list| {
    #[allow(unsafe_code)]
    let op_list = unsafe { &mut *op_list.get() };
    op_list.add_import(index, kind, namespace, operation);
  });
}

fn send_host_frame(mut payloads: Vec<Bytes>) -> Vec<Bytes> {
  let size = HOST_BUFFER.with(|cell| {
    #[allow(unsafe_code)]
    let buff = unsafe { &mut *cell.get() };
    let max = buff.capacity();
    let mut total = 0;
    let mut buff = Cursor::new(buff);
    while let Some(payload) = payloads.pop() {
      let len = payload.len() as u32;
      if (total + len as usize) > max {
        payloads.push(payload);
        break;
      }
      buff.write_all(&to_u24_bytes(len)).unwrap();
      buff.write_all(&payload).unwrap();
      total += 3 + len as usize;
    }
    total
  });
  #[allow(unsafe_code)]
  unsafe {
    tracing::trace!(size, "sending frame to host");
    _host_wasmrs_send(size);
  }
  payloads
}

#[allow(missing_docs)]
pub trait RequestFnF {
  fn fire_and_forget_wrapper(input: IncomingMono) -> Result<(), GenericError>;
}
#[allow(missing_docs)]
pub trait RequestResponse {
  fn request_response_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError>;
}
#[allow(missing_docs)]
pub trait RequestStream {
  fn request_stream_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError>;
}
#[allow(missing_docs)]
pub trait RequestChannel {
  fn request_channel_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError>;
}

#[allow(missing_docs)]
pub type ProcessReturnValue = Result<OutgoingStream, GenericError>;

fn register_handler<T>(
  kind: &'static std::thread::LocalKey<UnsafeCell<OperationMap<T>>>,
  ns: impl AsRef<str>,
  op: impl AsRef<str>,
  handler: T,
) -> u32 {
  kind.with(|cell| {
    #[allow(unsafe_code)]
    let buffer = unsafe { &mut *cell.get() };
    buffer.push((ns.as_ref().to_owned(), op.as_ref().to_owned(), Rc::new(handler)));
    (buffer.len() - 1) as _
  })
}

/// Register a request/response handler under the specified namespace and operation.
pub fn register_request_response(
  ns: impl AsRef<str>,
  op: impl AsRef<str>,
  handler: ProcessFactory<IncomingMono, OutgoingMono>,
) {
  let index = register_handler(&REQUEST_RESPONSE_HANDLERS, &ns, &op, handler);
  add_export(index, OperationType::RequestResponse, ns, op);
}

/// Register a request/stream handler under the specified namespace and operation.
pub fn register_request_stream(
  ns: impl AsRef<str>,
  op: impl AsRef<str>,
  handler: ProcessFactory<IncomingMono, OutgoingStream>,
) {
  let index = register_handler(&REQUEST_STREAM_HANDLERS, &ns, &op, handler);
  add_export(index, OperationType::RequestStream, ns, op);
}

/// Register a request/channel handler under the specified namespace and operation.
pub fn register_request_channel(
  ns: impl AsRef<str>,
  op: impl AsRef<str>,
  handler: ProcessFactory<IncomingStream, OutgoingStream>,
) {
  let index = register_handler(&REQUEST_CHANNEL_HANDLERS, &ns, &op, handler);
  add_export(index, OperationType::RequestChannel, ns, op);
}

/// Register a fire & forget handler under the specified namespace and operation.
pub fn register_fire_and_forget(ns: impl AsRef<str>, op: impl AsRef<str>, handler: ProcessFactory<IncomingMono, ()>) {
  let index = register_handler(&REQUEST_FNF_HANDLERS, &ns, &op, handler);
  add_export(index, OperationType::RequestFnF, ns, op);
}
