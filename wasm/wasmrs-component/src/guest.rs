use async_channel::Sender;
use bytes::{BufMut, Bytes, BytesMut};
use wasmrs_rsocket::flux::{FluxChannel, Signal};
// use rxrust::{
//     prelude::{Observer, SubscribeNext},
//     subject::LocalSubject,
// };
use std::cell::RefCell;
use std::{cell::UnsafeCell, collections::HashMap, rc::Rc, sync::atomic::Ordering};
use wasmrs_ringbuffer::{ReadOnlyRingBuffer, RingBuffer, VecRingBuffer};
use wasmrs_rsocket::{Flux, FragmentedPayload, Frame, FrameType, Metadata, Payload, PayloadError};

use std::sync::atomic::AtomicU32;

pub type GenericError = Box<dyn std::error::Error + Send + 'static>;
pub type NamespaceMap = HashMap<String, OperationMap>;
pub type OperationMap = HashMap<String, Rc<ProcessFactory>>;
pub type StreamMap = HashMap<u32, IncomingStream>;
pub type FragmentMap = HashMap<u32, FragmentedPayload>;
pub type ProcessFactory = fn(IncomingStream) -> std::result::Result<OutgoingStream, GenericError>;

pub type IncomingStream = FluxChannel<GuestPayload, PayloadError>;
pub type OutgoingStream = FluxChannel<Bytes, PayloadError>;

use yielding_executor::single_threaded as runtime;

#[derive(Clone)]
pub struct GuestPayload {
    pub stream_id: u32,
    pub data: Vec<u8>,
    pub metadata: Metadata,
}

impl GuestPayload {
    pub fn new(stream_id: u32, data: Vec<u8>, metadata: Metadata) -> Self {
        Self {
            stream_id,
            data,
            metadata,
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoHandler,
    HandlerFail(String),
    StringDecode,
    BufferRead,
}

pub struct GuestError(u32, String);

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error")
    }
}
impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::BufferRead
    }
}

thread_local! {
  static GUEST_BUFFER: UnsafeCell<VecRingBuffer<u8>> = UnsafeCell::new(VecRingBuffer::new());
  static HOST_BUFFER: UnsafeCell<VecRingBuffer<u8>> = UnsafeCell::new(VecRingBuffer::new());
  static MAX_HOST_FRAME_SIZE: AtomicU32 = AtomicU32::new(128);
  static STREAMS: RefCell<StreamMap> = RefCell::new(StreamMap::new());
  static FRAGMENTS: RefCell<FragmentMap> = RefCell::new(FragmentMap::new());
  static STREAM_ID: AtomicU32 = AtomicU32::new(2);
  static REQUEST_STREAM_HANDLERS: UnsafeCell<NamespaceMap> = UnsafeCell::new(NamespaceMap::new());
  static REQUEST_RESPONSE_HANDLERS: UnsafeCell<NamespaceMap> = UnsafeCell::new(NamespaceMap::new());
}

fn next_stream_id() -> u32 {
    STREAM_ID.with(|cell| cell.fetch_add(2, std::sync::atomic::Ordering::Relaxed))
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
    unsafe {
        __console_log(
            msg.as_ref().as_ptr() as usize as *const u8,
            msg.as_ref().len() as usize,
        );
    }
}

#[no_mangle]
extern "C" fn __wasmrs_init(
    guest_buffer_size: u32,
    host_buffer_size: u32,
    max_host_frame_len: u32,
) {
    //println!("in guest: __wasmrs_init");
    let guest_ptr = GUEST_BUFFER.with(|cell| {
        let buffer = unsafe { &mut *cell.get() };
        buffer.resize(guest_buffer_size as usize, || 0);
        buffer.as_ptr() as usize
    });
    let host_ptr = HOST_BUFFER.with(|cell| {
        let buffer = unsafe { &mut *cell.get() };
        buffer.resize(host_buffer_size as usize, || 0);
        buffer.as_ptr() as usize
    });
    MAX_HOST_FRAME_SIZE.with(|cell| cell.store(max_host_frame_len, Ordering::Relaxed));
    crate::init();
    unsafe {
        _host_wasmrs_init(guest_ptr, host_ptr);
    }
}

fn read_frame(read_pos: u32) -> Result<Bytes> {
    GUEST_BUFFER
        .with(|cell| {
            let mut buff = unsafe { &mut *cell.get() };
            buff.update_read_pos(read_pos as usize);
            wasmrs_rsocket::read_frame(&mut buff)
        })
        .map_err(|_| Error::BufferRead)
}

fn send_error_frame(stream_id: u32, code: u32, msg: impl AsRef<str>) {
    let err = Frame::new_error(stream_id, code, msg.as_ref());
    print(msg.as_ref());
    send_host_payload(err.encode());
}

#[no_mangle]
extern "C" fn __wasmrs_send(read_pos: u32) {
    //println!("in guest: __wasmrs_send");
    let read_result = read_frame(read_pos);
    if read_result.is_err() {
        send_error_frame(0, 0, "Could not read local buffer");
        return;
    }
    let bytes = read_result.unwrap();
    let frame = Frame::decode(bytes);

    if let Err((stream_id, _err)) = frame {
        send_error_frame(stream_id, 0, "Could not decode frame data");
        return;
    }
    let frame = frame.unwrap();

    let _ = handle_frame(frame);
    //println!("handled frame");
    //println!("queued tasks: {:?}", runtime::queued_tasks_count());
    runtime::run_while(move || {
        //println!("[[queued tasks: {:?}", runtime::queued_tasks_count());
        //println!("[[tasks: {:?}", runtime::tasks_count());
        runtime::queued_tasks_count() > 0
    });
}

fn handle_frame(frame: Frame) -> Result<()> {
    match frame {
        Frame::PayloadFrame(frame) => {
            update_fragment(
                frame.stream_id,
                FragmentedPayload::new(
                    FrameType::Payload,
                    Payload::new(frame.metadata, frame.data),
                ),
            );
            if frame.follows {
                return Ok(());
            }
            let mut complete_payload = get_fragment(frame.stream_id).unwrap();
            match complete_payload.frame_type {
                FrameType::RequestResponse => {
                    handle_request_response(
                        frame.stream_id,
                        complete_payload.metadata.to_vec(),
                        complete_payload.data.to_vec(),
                    )?;
                }
                FrameType::RequestFnf => {
                    todo!();
                }
                FrameType::RequestStream => {
                    handle_request_stream(
                        frame.stream_id,
                        complete_payload.metadata.to_vec(),
                        complete_payload.data.to_vec(),
                    )?;
                }
                FrameType::RequestChannel => {
                    handle_request_channel(
                        frame.stream_id,
                        complete_payload.metadata.to_vec(),
                        complete_payload.data.to_vec(),
                    )?;
                }
                FrameType::Payload => {
                    if frame.next {
                        let mut stream = get_stream(frame.stream_id).unwrap();
                        let metadata = parse_metadata(&mut complete_payload.metadata)?;
                        stream.send(GuestPayload::new(
                            frame.stream_id,
                            complete_payload.data.to_vec(),
                            metadata,
                        ));
                    }
                }
                _ => unreachable!(),
            }
            if frame.complete {
                end_stream(frame.stream_id);
            }
        }

        Frame::Cancel(frame) => {
            end_stream(frame.stream_id);
        }
        Frame::ErrorFrame(frame) => {
            let mut stream = get_stream(frame.stream_id).unwrap();
            stream.error(PayloadError::new(frame.code, frame.data));
            end_stream(frame.stream_id);
        }
        Frame::RequestN(frame) => {
            let stream = get_stream(frame.stream_id).unwrap();
            // Stream do request?
            // stream.do_request(frame.n);
            todo!();
        }
        Frame::RequestResponse(frame) => {
            if !frame.0.follows {
                handle_request_response(
                    frame.0.stream_id,
                    frame.0.metadata.to_vec(),
                    frame.0.data.to_vec(),
                )?;
            }
        }
        Frame::RequestFnF(_) => todo!(),
        Frame::RequestStream(frame) => {
            if !frame.0.follows {
                handle_request_stream(
                    frame.0.stream_id,
                    frame.0.metadata.to_vec(),
                    frame.0.data.to_vec(),
                )?;
            }
        }
        Frame::RequestChannel(frame) => {
            if !frame.0.follows {
                handle_request_channel(
                    frame.0.stream_id,
                    frame.0.metadata.to_vec(),
                    frame.0.data.to_vec(),
                )?;
            }
        }
    };
    Ok(())
}

fn send_host_payload(payload: Bytes) {
    //println!("sending host payload");
    let host_start = HOST_BUFFER.with(|cell| {
        let buff = unsafe { &mut *cell.get() };
        let start = buff.get_write_pos();
        let len = payload.len() as u32;
        buff.write(len.to_be_bytes());
        buff.write(payload);
        buff.update_write_pos(4 + len as usize);
        start
    });
    unsafe {
        _host_wasmrs_send(host_start);
    }
}

fn update_fragment(id: u32, mut fragment: FragmentedPayload) {
    FRAGMENTS.with(|cell| match cell.borrow_mut().get_mut(&id) {
        Some(existing_fragment) => {
            existing_fragment.metadata.put(&mut fragment.metadata);
            existing_fragment.data.put(&mut fragment.data);
        }
        None => {
            cell.borrow_mut().insert(id, fragment);
        }
    })
}

fn parse_metadata(bytes: &[u8]) -> Result<Metadata> {
    let (namespace, nslen) = read_string(0, bytes)?;
    let (operation, oplen) = read_string(nslen, bytes)?;
    let (instance, _) = read_data(nslen + oplen, bytes)?;
    Ok(Metadata {
        namespace,
        operation,
        instance: instance.into(),
    })
}

// Read a string chunk whose length is denoted by a u16 prefix.
fn read_string(start: usize, buffer: &[u8]) -> Result<(String, usize)> {
    let (bytes, len) = read_data(start, buffer)?;
    Ok((
        String::from_utf8(bytes).map_err(|_| Error::StringDecode)?,
        len,
    ))
}

// Read a data chunk whose length is denoted by a u16 prefix.
fn read_data(start: usize, buffer: &[u8]) -> Result<(Vec<u8>, usize)> {
    let len_bytes: &mut [u8] = &mut [0_u8; 2];
    len_bytes.copy_from_slice(&buffer[start..start + 2]);
    let len = wasmrs_rsocket::from_u16_bytes(len_bytes) as usize;
    let mut data_bytes = vec![0_u8; len];
    data_bytes.copy_from_slice(&buffer[start + 2..start + 2 + len]);
    Ok((data_bytes, 2 + len))
}

fn get_fragment(id: u32) -> Option<FragmentedPayload> {
    FRAGMENTS.with(|cell| cell.borrow_mut().remove(&id))
}

fn get_stream(id: u32) -> Option<IncomingStream> {
    STREAMS.with(|cell| cell.borrow().get(&id).cloned())
}

fn new_stream(id: u32) -> IncomingStream {
    let stream = IncomingStream::new();
    let inner = stream.clone();
    STREAMS.with(|cell| cell.borrow_mut().insert(id, inner));
    stream
}

fn remove_stream(id: u32) {
    STREAMS.with(|cell| cell.borrow_mut().remove(&id));
}

fn end_stream(id: u32) {
    STREAMS.with(|cell| match cell.borrow_mut().remove(&id) {
        Some(mut stream) => stream.complete(),
        None => print("ending stream without a stream available"),
    });
}

fn get_process_handler(
    kind: &'static std::thread::LocalKey<UnsafeCell<NamespaceMap>>,
    namespace: &str,
    op: &str,
) -> Option<Rc<ProcessFactory>> {
    kind.with(|cell| {
        let buffer = unsafe { &*cell.get() };
        buffer
            .get(namespace)
            .and_then(|opmap| opmap.get(op).cloned())
    })
}

fn handle_request_response(
    stream_id: u32,
    mut metadata_bytes: Vec<u8>,
    data: Vec<u8>,
) -> Result<()> {
    let metadata = parse_metadata(&metadata_bytes)?;
    let handler = get_process_handler(
        &REQUEST_RESPONSE_HANDLERS,
        &metadata.namespace,
        &metadata.operation,
    )
    .ok_or(Error::NoHandler)?;
    let mut incoming = new_stream(stream_id);
    let result = handler(incoming.clone()).map_err(|e| Error::HandlerFail(e.to_string()));

    yielding_executor::single_threaded::spawn(async move {
        //println!("in outgoing stream processor");

        match result {
            Ok(outgoing) => {
                //println!("waiting for outgoing signals");

                while let Ok(Some(signal)) = outgoing.recv().await {
                    //println!("got signal: {:?}", signal);
                    match signal {
                        Ok(payload) => send_host_payload(
                            Frame::new_payload(
                                stream_id,
                                Payload::new(Bytes::from(metadata_bytes.clone()), payload),
                                0,
                            )
                            .encode(),
                        ),
                        Err(e) => send_error_frame(stream_id, e.code, e.msg),
                    }
                }
            }
            Err(e) => send_error_frame(
                stream_id,
                wasmrs_rsocket::ErrorCode::ApplicationError.into(),
                e.to_string(),
            ),
        };
        //println!("Done processing output");
    });
    //println!("outgoing task started");
    let _ = incoming.send(GuestPayload::new(stream_id, data.to_vec(), metadata));
    //println!("done with reqres");

    Ok(())
}

fn handle_request_stream(stream_id: u32, mut metadata: Vec<u8>, data: Vec<u8>) -> Result<()> {
    let metadata = parse_metadata(&mut metadata)?;
    let handler = get_process_handler(
        &REQUEST_STREAM_HANDLERS,
        &metadata.namespace,
        &metadata.operation,
    )
    .ok_or(Error::NoHandler)?;
    let incoming = new_stream(stream_id);
    // match handler(incoming).map_err(|e| Error::HandlerFail(e.to_string())) {
    //     Ok(outgoing) => {
    //         outgoing.subscribe(|wat| {});
    //     }
    //     Err(e) => send_error_frame(
    //         stream_id,
    //         wasmrs_rsocket::ErrorCode::ApplicationError.into(),
    //         e.to_string(),
    //     ),
    // };
    todo!();
    Ok(())
}

fn handle_request_channel(stream_id: u32, mut metadata: Vec<u8>, data: Vec<u8>) -> Result<()> {
    let metadata = parse_metadata(&mut metadata);
    Ok(())
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
        let buffer = unsafe { &mut *cell.get() };
        let ops = buffer
            .entry(ns.as_ref().to_owned())
            .or_insert_with(HashMap::new);
        ops.insert(op.as_ref().to_owned(), Rc::new(handler));
    })
}

// Generated
