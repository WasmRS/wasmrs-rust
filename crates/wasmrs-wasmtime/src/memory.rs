use bytes::Bytes;
use wasmrs_ringbuffer::SharedReadOnlyRingBuffer;
use wasmtime::{AsContext, Caller, Memory, StoreContext};

use crate::errors::Error;

pub(crate) fn get_vec_from_memory<'a, T: 'a>(
    store: impl Into<StoreContext<'a, T>>,
    mem: Memory,
    ptr: i32,
    len: i32,
) -> Vec<u8> {
    let data = mem.data(store);
    data[ptr as usize..(ptr + len) as usize].to_vec()
}

pub(crate) fn get_caller_memory<T>(caller: &mut Caller<T>) -> Memory {
    let memory = caller
        .get_export("memory")
        .map(|e| e.into_memory().unwrap());
    memory.unwrap()
}

pub(crate) fn get_vec_from_ringbuffer<'a, T: 'a>(
    store: impl Into<StoreContext<'a, T>>,
    mem: Memory,
    recv_pos: u32,
    ring_start: u32,
    ring_len: u32,
) -> super::Result<Bytes> {
    let data = mem.data(store);
    let buff = SharedReadOnlyRingBuffer::new(
        data,
        ring_start as usize,
        ring_len as usize,
        recv_pos as usize,
    );
    wasmrs::read_frame(buff).map_err(|_| Error::GuestMemory)
}

pub(crate) fn write_bytes_to_memory(
    store: impl AsContext,
    memory: Memory,
    buffer: &[u8],
    recv_pos: u32,
    ring_start: u32,
    ring_len: u32,
) -> u32 {
    let len = buffer.len();
    let remaining: usize = (ring_len - recv_pos) as _;
    let start_offset = ring_start + recv_pos;

    #[allow(unsafe_code)]
    unsafe {
        let mut guest_ptr = memory.data_ptr(&store).offset(start_offset as isize);
        if len > remaining {
            guest_ptr.copy_from(buffer.as_ptr(), remaining);
            guest_ptr = memory.data_ptr(store).offset(ring_start as isize);
            guest_ptr.copy_from(buffer.as_ptr().add(remaining), len - remaining);
        } else {
            guest_ptr.copy_from(buffer.as_ptr(), len);
        }
    }
    len as u32
}
