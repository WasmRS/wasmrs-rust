use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::util::{from_u24_bytes, to_u24_bytes};

use super::Metadata;

impl Metadata {
  /// Create a new [Metadata] object for the specified stream_id.
  pub fn new(index: u32) -> Metadata {
    Metadata {
      index: Some(index),
      extra: None,
    }
  }

  /// Create a new [Metadata] object for the specified stream_id.
  pub fn new_extra(index: u32, extra: Bytes) -> Metadata {
    Metadata {
      index: Some(index),
      extra: Some(extra),
    }
  }

  #[must_use]
  /// Encode the [Metadata] object into bytes for sending in a [crate::Frame].
  pub fn encode(self) -> Bytes {
    let custom_mime_len = 4;
    let our_len: u32 = self
      .index
      .map_or(0, |_| 8 + self.extra.as_ref().map(|e| e.len()).unwrap_or(0) as u32);

    let mut bytes = BytesMut::with_capacity(custom_mime_len + our_len as usize);
    bytes.fill(0);
    bytes.put_u8(0xca);
    bytes.put(to_u24_bytes(our_len));

    if let Some(index) = &self.index {
      bytes.put((index).to_be_bytes().as_slice());
      bytes.put([0u8, 0, 0, 0].as_slice()); // reserved

      if let Some(extra) = self.extra {
        bytes.put(extra);
      }
    }

    bytes.freeze()
  }

  /// Decode bytes into [Metadata] object
  pub fn decode(bytes: &mut Bytes) -> Result<Self, crate::Error> {
    if bytes.is_empty() {
      return Ok(Self {
        index: None,
        extra: None,
      });
    }

    if bytes[0] == 0xca {
      // new, RSocket-aligned metadata

      let _mime_type = bytes.get_u8();

      let _mime_len = from_u24_bytes(&bytes.split_to(3)) as usize;

      let index = bytes.get_u32();
      let _reserved = bytes.get_u32();

      let extra = if bytes.is_empty() {
        None
      } else {
        Some(bytes.split_to(bytes.remaining()))
      };

      Ok(Self {
        index: Some(index),
        extra,
      })
    } else {
      let index = bytes.get_u32();

      let _reserved = bytes.get_u32();
      let extra = if bytes.is_empty() { None } else { Some(bytes.clone()) };
      let md = Metadata {
        index: Some(index),
        extra,
      };
      Ok(md)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use anyhow::Result;

  #[test]
  fn test_rt() -> Result<()> {
    let md = Metadata::new(32);
    let bytes = md.clone().encode();
    let mut bytes = bytes.clone();
    let md2 = Metadata::decode(&mut bytes)?;

    assert_eq!(md, md2);

    Ok(())
  }

  #[test]
  fn test_old() -> Result<()> {
    let md = Metadata::new(48);

    let mut bytes: Bytes = vec![0, 0, 0, 0x30, 0, 0, 0, 0].into();
    let md2 = Metadata::decode(&mut bytes)?;

    assert_eq!(md, md2);

    Ok(())
  }

  #[test]
  fn test_new() -> Result<()> {
    let md = Metadata::new(48);
    let mut bytes: Bytes = vec![0xca, 0, 0, 8, 0, 0, 0, 0x30, 0, 0, 0, 0].into();
    let md2 = Metadata::decode(&mut bytes)?;

    assert_eq!(md, md2);

    Ok(())
  }

  #[test]
  fn test_new_extra() -> Result<()> {
    let md = Metadata::new_extra(48, b"hello".to_vec().into());
    let mut bytes = md.clone().encode();

    let md2 = Metadata::decode(&mut bytes)?;

    assert_eq!(md, md2);

    Ok(())
  }
}
