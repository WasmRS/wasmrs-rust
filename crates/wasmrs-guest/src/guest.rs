pub use wasmrs::runtime::spawn;
use wasmrs::runtime::{exhaust_pool, Receiver};

use std::cell::RefCell;
use std::{cell::UnsafeCell, collections::HashMap, rc::Rc, sync::atomic::Ordering};
use wasmrs_ringbuffer::{ReadOnlyRingBuffer, RingBuffer, VecRingBuffer};

use std::sync::atomic::AtomicU32;
pub use wasmrs::{
    flux::FluxChannel, flux::FluxStream, Flux, Frame, Metadata, Payload, PayloadError,
};

pub type GenericError = Box<dyn std::error::Error + Send + 'static>;
pub type NamespaceMap = HashMap<String, OperationMap>;
pub type OperationMap = HashMap<String, Rc<ProcessFactory>>;
pub type StreamMap = HashMap<u32, IncomingStream>;
pub type ProcessFactory = fn(IncomingStream) -> std::result::Result<OutgoingStream, GenericError>;

pub type IncomingStream = FluxChannel<ParsedPayload, PayloadError>;
pub type OutgoingStream = FluxChannel<Payload, PayloadError>;

use crate::error::Error;
use crate::server::WasmServer;
pub use bytes::Bytes;

pub use futures_util::{stream::select_all, StreamExt};
pub use wasmrs_codec::messagepack::{deserialize, serialize};

type Result<T> = std::result::Result<T, Error>;

thread_local! {
  static GUEST_BUFFER: UnsafeCell<VecRingBuffer<u8>> = UnsafeCell::new(VecRingBuffer::new());
  static HOST_BUFFER: UnsafeCell<VecRingBuffer<u8>> = UnsafeCell::new(VecRingBuffer::new());
  static MAX_HOST_FRAME_SIZE: AtomicU32 = AtomicU32::new(128);
  static STREAMS: RefCell<StreamMap> = RefCell::new(StreamMap::new());
  static STREAM_ID: AtomicU32 = AtomicU32::new(2);
  pub(crate) static REQUEST_RESPONSE_HANDLERS: UnsafeCell<NamespaceMap> = UnsafeCell::new(NamespaceMap::new());
  static MANAGER: UnsafeCell<wasmrs::manager::SocketManager> = UnsafeCell::new(wasmrs::manager::SocketManager::new(WasmServer{}));
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
    /// The host's exported __console_log function.
    pub(crate) fn __console_log(ptr: *const u8, len: usize);
    /// The host's exported __host_call function.
    #[link_name = "__init_buffers"]
    pub(crate) fn _host_wasmrs_init(guest_buffer_ptr: usize, host_buffer_ptr: usize);
    /// The host's exported __host_response function.
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
    println!("in guest: __wasmrs_init");
    let guest_ptr = GUEST_BUFFER.with(|cell| {
        #[allow(unsafe_code)]
        let buffer = unsafe { &mut *cell.get() };
        buffer.resize(guest_buffer_size as usize, || 0);
        buffer.as_ptr()
    });
    let rx = MANAGER.with(|cell| {
        #[allow(unsafe_code)]
        let manager = unsafe { &mut *cell.get() };
        manager.host_buffer().update_size(host_buffer_size);
        manager.guest_buffer().update_size(guest_buffer_size);
        manager.take_rx().unwrap()
    });
    spawn_writer(rx);

    let host_ptr = HOST_BUFFER.with(|cell| {
        #[allow(unsafe_code)]
        let buffer = unsafe { &mut *cell.get() };
        buffer.resize(host_buffer_size as usize, || 0);
        buffer.as_ptr()
    });
    MAX_HOST_FRAME_SIZE.with(|cell| cell.store(max_host_frame_len, Ordering::Relaxed));
    #[allow(unsafe_code)]
    unsafe {
        _host_wasmrs_init(guest_ptr, host_ptr);
    }
}

fn spawn_writer(mut rx: Receiver<Frame>) {
    spawn(async move {
        while let Some(frame) = rx.recv().await {
            send_host_payload(frame.encode());
        }
    });
}

fn read_frame(read_pos: u32) -> Result<Bytes> {
    GUEST_BUFFER
        .with(|cell| {
            #[allow(unsafe_code)]
            let mut buff = unsafe { &mut *cell.get() };
            buff.update_read_pos(read_pos as usize);
            wasmrs::read_frame(&mut buff)
        })
        .map_err(|_| Error::BufferRead)
}

fn send_error_frame(stream_id: u32, code: u32, msg: impl AsRef<str>) {
    let err = Frame::new_error(stream_id, code, msg.as_ref());
    print(msg.as_ref());
    send_host_payload(err.encode());
}

#[allow(unsafe_code)]
#[no_mangle]
extern "C" fn __wasmrs_send(read_pos: u32) {
    println!("in guest: __wasmrs_send");
    let read_result = read_frame(read_pos);
    if read_result.is_err() {
        send_error_frame(0, 0, "Could not read local buffer");
        return;
    }
    let bytes = read_result.unwrap();
    println!("got frame: {:?}", bytes);
    let frame = Frame::decode(bytes);

    if let Err((stream_id, _err)) = frame {
        send_error_frame(stream_id, 0, "Could not decode frame data");
        return;
    }

    MANAGER.with(|cell| {
        let manager = unsafe { &mut *cell.get() };
        match frame {
            Ok(frame) => {
                let _ = manager.process_once(frame);
            }
            Err(_e) => {
                println!("error at handler");
            }
        }
    });

    exhaust_pool();
}

fn send_host_payload(payload: Bytes) {
    println!("sending host payload");
    let host_start = HOST_BUFFER.with(|cell| {
        #[allow(unsafe_code)]
        let buff = unsafe { &mut *cell.get() };
        let start = buff.get_write_pos();
        let len = payload.len() as u32;
        buff.write(len.to_be_bytes());
        buff.write(payload);
        buff.update_write_pos(4 + len as usize);
        start
    });
    #[allow(unsafe_code)]
    unsafe {
        _host_wasmrs_send(host_start);
    }
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
        let ops = buffer
            .entry(ns.as_ref().to_owned())
            .or_insert_with(HashMap::new);
        ops.insert(op.as_ref().to_owned(), Rc::new(handler));
    });
}
