use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::Metadata;

impl Metadata {
  /// Create a new [Metadata] object for the specified stream_id.
  pub fn new(index: u32) -> Metadata {
    Metadata { index, extra: None }
  }

  /// Create a new [Metadata] object for the specified stream_id.
  pub fn new_extra(index: u32, extra: Bytes) -> Metadata {
    Metadata {
      index,
      extra: Some(extra),
    }
  }

  #[must_use]
  /// Encode the [Metadata] object into bytes for sending in a [crate::Frame].
  pub fn encode(self) -> Bytes {
    let len = 8;
    let mut bytes = BytesMut::with_capacity(len);
    bytes.fill(0);
    bytes.put((self.index).to_be_bytes().as_slice());
    bytes.put([0u8, 0, 0, 0].as_slice());

    debug_assert_eq!(bytes.len(), len, "encoded metadata is not the correct length.");

    if let Some(extra) = self.extra {
      bytes.put(extra);
    }

    bytes.freeze()
  }

  /// Decode bytes into [Metadata] object
  pub fn decode(bytes: &mut Bytes) -> Result<Self, crate::Error> {
    let index = bytes.get_u32();

    let _reserved = bytes.get_u32();
    let extra = if bytes.is_empty() { None } else { Some(bytes.clone()) };
    Ok(Metadata { index, extra })
  }
}
