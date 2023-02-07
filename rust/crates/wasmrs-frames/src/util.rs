use bytes::{BufMut, Bytes, BytesMut};

#[must_use]
pub(crate) fn from_u32_bytes(bytes: &[u8]) -> u32 {
  assert!(bytes.len() == 4, "Need 4 bytes to convert to u32");
  let mut num_parts: [u8; 4] = Default::default();

  num_parts[0..4].copy_from_slice(bytes);

  u32::from_be_bytes(num_parts)
}

#[must_use]
pub(crate) fn from_u16_bytes(bytes: &[u8]) -> u16 {
  assert!(bytes.len() == 2, "Need two bytes to convert to u16");
  let mut num_parts: [u8; 2] = Default::default();

  num_parts[0..2].copy_from_slice(bytes);

  u16::from_be_bytes(num_parts)
}

#[must_use]
pub(crate) fn from_u24_bytes(bytes: &[u8]) -> u32 {
  assert!(bytes.len() == 3, "Need three bytes to convert to u24");
  let mut num_parts: [u8; 4] = Default::default();

  num_parts[1..4].copy_from_slice(bytes);

  u32::from_be_bytes(num_parts)
}

#[must_use]
/// Convert a [u32] to a `u24` represented in bytes.
pub(crate) fn to_u24_bytes(num: u32) -> Bytes {
  let mut num_parts = BytesMut::with_capacity(3);

  num_parts.put(&num.to_be_bytes()[1..4]);

  num_parts.freeze()
}
