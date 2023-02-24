use bytes::Bytes;
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
  let memory = caller.get_export("memory").map(|e| e.into_memory().unwrap());
  memory.unwrap()
}

pub(crate) fn read_frame<'a, T: 'a>(
  store: impl Into<StoreContext<'a, T>>,
  mem: Memory,
  ptr: usize,
  read_until: usize,
) -> super::Result<Bytes> {
  let data = mem.data(store);
  let buff = &data[ptr..(ptr + read_until)];
  trace!("{:?}", &data[ptr..(ptr + read_until)]);
  wasmrs::util::read_frame(buff).map_err(|_| Error::GuestMemory)
}

pub(crate) fn write_bytes_to_memory(
  store: impl AsContext,
  memory: Memory,
  buffer: &[u8],
  buffer_start: u32,
  buffer_len: u32,
) -> u32 {
  let len = buffer.len();

  #[allow(unsafe_code)]
  unsafe {
    let guest_ptr = memory.data_ptr(&store).offset(buffer_start as isize);
    assert!(
      len <= buffer_len as usize,
      "Writing more data than guest buffer can store."
    );
    guest_ptr.copy_from(buffer.as_ptr(), len);
  }
  len as u32
}
