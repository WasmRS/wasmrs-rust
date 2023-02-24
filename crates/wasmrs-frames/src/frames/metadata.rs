use bytes::{BufMut, Bytes, BytesMut};

use super::Metadata;

impl Metadata {
  /// Create a new [Metadata] object for the specified stream_id.
  pub fn new(index: u32) -> Metadata {
    Metadata { index }
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
    bytes.freeze()
  }
}
