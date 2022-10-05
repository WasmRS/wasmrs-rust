pub use wasmrs::runtime::spawn;
use wasmrs::runtime::{exhaust_pool, UnboundedReceiver};
use wasmrs::SocketSide;

use std::io::{Cursor, Write};
use std::{cell::UnsafeCell, rc::Rc, sync::atomic::Ordering};

use std::sync::atomic::AtomicU32;
pub use wasmrs::{flux::*, Frame, Metadata, Payload, PayloadError};

pub type GenericError = Box<dyn std::error::Error + Send + 'static>;
pub type OperationMap = Vec<(String, String, Rc<ProcessFactory>)>;
pub type ProcessFactory = fn(IncomingStream) -> std::result::Result<OutgoingStream, GenericError>;

pub type IncomingStream = FluxReceiver<ParsedPayload, PayloadError>;
pub type OutgoingStream = Flux<Payload, PayloadError>;

use crate::error::Error;
use crate::server::WasmServer;
pub use bytes::Bytes;

pub use futures_util::{stream::select_all, StreamExt};
pub use wasmrs_codec::messagepack::{deserialize, serialize};

type Result<T> = std::result::Result<T, Error>;

thread_local! {
  static GUEST_BUFFER: UnsafeCell<Vec<u8>> = UnsafeCell::new(Vec::new());
  static HOST_BUFFER: UnsafeCell<Vec<u8>> = UnsafeCell::new(Vec::new());
  static MAX_HOST_FRAME_SIZE: AtomicU32 = AtomicU32::new(128);
  pub(crate) static REQUEST_RESPONSE_HANDLERS: UnsafeCell<OperationMap> = UnsafeCell::new(OperationMap::new());
  static MANAGER: UnsafeCell<wasmrs::WasmSocket> = UnsafeCell::new(wasmrs::WasmSocket::new(WasmServer{}, SocketSide::Guest));
}

#[allow(missing_debug_implementations)]
pub struct ParsedPayload {
    pub metadata: Metadata,
    pub data: Bytes,
}

impl TryFrom<Payload> for ParsedPayload {
    type Error = Error;

    fn try_from(value: Payload) -> Result<Self> {
        Ok(ParsedPayload {
            metadata: value.parse_metadata()?,
            data: value.data.unwrap_or_default(),
        })
    }
}

#[link(wasm_import_module = "wasmrs")]
extern "C" {
    pub(crate) fn __console_log(ptr: *const u8, len: usize);
    #[link_name = "__init_buffers"]
    pub(crate) fn _host_wasmrs_init(guest_buffer_ptr: usize, host_buffer_ptr: usize);
    #[link_name = "__send"]
    pub(crate) fn _host_wasmrs_send(recv_ptr: usize);
}

fn print(msg: impl AsRef<str>) {
    #[allow(unsafe_code)]
    unsafe {
        __console_log(
            msg.as_ref().as_ptr() as usize as *const u8,
            msg.as_ref().len(),
        );
    }
}

pub fn init(guest_buffer_size: u32, host_buffer_size: u32, max_host_frame_len: u32) {
    tracing::trace!("guest::init() called");

    let guest_ptr = GUEST_BUFFER.with(|cell| {
        #[allow(unsafe_code)]
        let buffer = unsafe { &mut *cell.get() };
        buffer.resize(guest_buffer_size as usize, 0);
        buffer.as_ptr()
    });
    let rx = MANAGER.with(|cell| {
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

fn spawn_writer(mut rx: UnboundedReceiver<Frame>) {
    spawn(async move {
        while let Some(frame) = rx.recv().await {
            send_host_payload(vec![frame.encode()]);
        }
    });
}

fn read_frames(read_until: u32) -> Result<Vec<Bytes>> {
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
    print(msg.as_ref());
    send_host_payload(vec![err.encode()]);
}

#[allow(unsafe_code)]
#[no_mangle]
extern "C" fn __wasmrs_send(read_until: u32) {
    tracing::trace!("__wasmrs_send() called");
    let read_result = read_frames(read_until);
    if read_result.is_err() {
        send_error_frame(0, 0, "Could not read local buffer");
        return;
    }
    let bytes_list = read_result.unwrap();

    MANAGER.with(|cell| {
        let manager = unsafe { &mut *cell.get() };
        for bytes in bytes_list {
            let frame = Frame::decode(bytes);
            match frame {
                Ok(frame) => {
                    let _ = manager.process_once(frame);
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

fn send_host_payload(mut payloads: Vec<Bytes>) -> Vec<Bytes> {
    let host_start = HOST_BUFFER.with(|cell| {
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
            buff.write_all(&len.to_be_bytes()).unwrap();
            buff.write_all(&payload).unwrap();
            total += 4 + len as usize;
        }
        total
    });
    #[allow(unsafe_code)]
    unsafe {
        _host_wasmrs_send(host_start);
    }
    payloads
}

pub trait Process {
    fn start(input_stream: IncomingStream) -> ProcessReturnValue;
}

pub type ProcessReturnValue = std::result::Result<OutgoingStream, GenericError>;

pub fn register_request_response(
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: ProcessFactory,
) {
    REQUEST_RESPONSE_HANDLERS.with(|cell| {
        #[allow(unsafe_code)]
        let buffer = unsafe { &mut *cell.get() };
        buffer.push((
            ns.as_ref().to_owned(),
            op.as_ref().to_owned(),
            Rc::new(handler),
        ));
    });
}
