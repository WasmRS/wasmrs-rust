use std::{
    io::Read,
    sync::{
        atomic::{AtomicI64, AtomicU32, Ordering},
        Arc,
    },
};

use bytes::{BufMut, Bytes, BytesMut};

use crate::Error;

#[must_use]
pub fn from_u32_bytes(bytes: &[u8]) -> u32 {
    assert!(bytes.len() == 4, "Need 4 bytes to convert to u32");
    let mut num_parts: [u8; 4] = Default::default();

    num_parts[0..4].copy_from_slice(bytes);

    u32::from_be_bytes(num_parts)
}

#[must_use]
pub fn from_u16_bytes(bytes: &[u8]) -> u16 {
    assert!(bytes.len() == 2, "Need two bytes to convert to u16");
    let mut num_parts: [u8; 2] = Default::default();

    num_parts[0..2].copy_from_slice(bytes);

    u16::from_be_bytes(num_parts)
}

#[must_use]
pub fn from_u24_bytes(bytes: &[u8]) -> u32 {
    assert!(bytes.len() == 3, "Need three bytes to convert to u24");
    let mut num_parts: [u8; 4] = Default::default();

    num_parts[1..4].copy_from_slice(bytes);

    u32::from_be_bytes(num_parts)
}

#[must_use]
pub fn to_u24_bytes(num: u32) -> Bytes {
    let mut num_parts = BytesMut::with_capacity(3);

    num_parts.put(&num.to_be_bytes()[1..4]);

    num_parts.freeze()
}

// Read a string chunk whose length is denoted by a u16 prefix.
pub fn read_string(start: usize, buffer: &[u8]) -> Result<(String, usize), Error> {
    let (bytes, len) = read_data(start, buffer)?;
    Ok((
        String::from_utf8(bytes).map_err(|_| Error::StringDecode)?,
        len,
    ))
}

// Read a data chunk whose length is denoted by a u16 prefix.
pub fn read_data(start: usize, buffer: &[u8]) -> Result<(Vec<u8>, usize), Error> {
    let len_bytes: &mut [u8] = &mut [0_u8; 2];
    len_bytes.copy_from_slice(&buffer[start..start + 2]);
    let len = from_u16_bytes(len_bytes) as usize;
    let mut data_bytes = vec![0_u8; len];
    data_bytes.copy_from_slice(&buffer[start + 2..start + 2 + len]);
    Ok((data_bytes, 2 + len))
}

pub fn read_frame(mut buf: impl Read) -> std::io::Result<Bytes> {
    let mut len_bytes = [0u8; 4];
    buf.read_exact(&mut len_bytes)?;
    let len = from_u32_bytes(&len_bytes);

    let mut frame = vec![0; len as usize];
    buf.read_exact(&mut frame)?;
    Ok(frame.into())
}

#[derive(Debug, Clone)]
pub struct StreamID {
    inner: Arc<AtomicU32>,
}

impl StreamID {
    pub(crate) fn new(value: u32) -> StreamID {
        let inner = Arc::new(AtomicU32::new(value));
        StreamID { inner }
    }

    #[allow(clippy::must_use_candidate)]
    pub fn next(&self) -> u32 {
        let counter = self.inner.clone();
        counter.fetch_add(2, Ordering::SeqCst)
    }
}

impl From<u32> for StreamID {
    fn from(v: u32) -> StreamID {
        StreamID::new(v)
    }
}

#[derive(Debug, Clone)]
#[must_use]
pub struct Counter {
    #[allow(unused)]
    inner: Arc<AtomicI64>,
}

impl Counter {
    #[allow(unused)]

    pub fn new(value: i64) -> Counter {
        Counter {
            inner: Arc::new(AtomicI64::new(value)),
        }
    }

    #[must_use]
    pub fn count_down(&self) -> i64 {
        self.inner.fetch_add(-1, Ordering::SeqCst) - 1
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::read_frame;

    #[test]
    fn test_read_frame() -> Result<()> {
        let mut buf: &[u8] = &[0, 0, 0, 4, 1, 2, 3, 4];
        let frame = read_frame(&mut buf)?;
        assert_eq!(frame, vec![1, 2, 3, 4]);

        Ok(())
    }
}
