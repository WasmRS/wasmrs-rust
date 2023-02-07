use bytes::{BufMut, Bytes, BytesMut};

use super::{Error, FrameFlags, FrameHeader, FrameType, RSocketFlags, RSocketFrame};
use crate::util::{from_u24_bytes, to_u24_bytes};
use crate::{Frame, Payload};

/// A Payload frame.
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[cfg_attr(feature = "serde", derive(Clone))]
#[must_use]
pub struct PayloadFrame {
  /// The stream ID this frame belongs to.
  pub stream_id: u32,
  /// Any metadata associated with the Payload as raw bytes.
  pub metadata: Bytes,
  /// The actual payload data as raw bytes.
  pub data: Bytes,
  /// Whether this payload is broken up into multiple frames.
  pub follows: bool,
  /// Whether or not this frame is the last frame in a stream.
  pub complete: bool,
  /// TODO
  pub next: bool,
}

impl PayloadFrame {
  pub(crate) fn from_payload(stream_id: u32, payload: Payload, flags: FrameFlags) -> Self {
    Self {
      stream_id,
      metadata: payload.metadata.unwrap_or_default(),
      data: payload.data.unwrap_or_default(),
      follows: flags.flag_follows(),
      complete: flags.flag_complete(),
      next: flags.flag_next(),
    }
  }
}

impl RSocketFrame<PayloadFrame> for PayloadFrame {
  const FRAME_TYPE: FrameType = FrameType::Payload;

  fn stream_id(&self) -> u32 {
    self.stream_id
  }

  fn decode_all(mut buffer: Bytes) -> Result<Self, Error> {
    let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
    Self::decode_frame(&header, buffer)
  }

  fn decode_frame(header: &FrameHeader, mut buffer: Bytes) -> Result<Self, Error> {
    Self::check_type(header)?;

    let metadata = if header.has_metadata() {
      let metadata_len = from_u24_bytes(&buffer.split_to(3)) as usize;
      buffer.split_to(metadata_len)
    } else {
      Bytes::new()
    };
    let payload: Bytes = buffer;

    Ok(PayloadFrame {
      stream_id: header.stream_id(),
      metadata,
      data: payload,
      follows: header.has_follows(),
      complete: header.has_complete(),
      next: header.has_next(),
    })
  }

  fn encode(self) -> Bytes {
    let header = self.gen_header().encode();
    let (mlen, md) = if self.metadata.is_empty() {
      (Bytes::new(), Bytes::new())
    } else {
      (to_u24_bytes(self.metadata.len() as u32), self.metadata)
    };
    let data = self.data;
    let mut bytes = BytesMut::with_capacity(Frame::LEN_HEADER + mlen.len() + md.len() + data.len());
    bytes.put(header);
    bytes.put(mlen);
    bytes.put(md);
    bytes.put(data);
    bytes.freeze()
  }

  fn gen_header(&self) -> FrameHeader {
    FrameHeader::new(self.stream_id, FrameType::Payload, self.get_flag())
  }

  fn get_flag(&self) -> FrameFlags {
    let mut flags = 0;
    if !self.metadata.is_empty() {
      flags |= Frame::FLAG_METADATA;
    }
    if self.complete {
      flags |= Frame::FLAG_COMPLETE;
    }
    if self.next {
      flags |= Frame::FLAG_NEXT;
    }
    if self.follows {
      flags |= Frame::FLAG_FOLLOW;
    }
    flags
  }
}

impl From<PayloadFrame> for Payload {
  fn from(req: PayloadFrame) -> Self {
    Payload {
      metadata: Some(req.metadata),
      data: Some(req.data),
    }
  }
}

#[cfg(test)]
mod test {
  use anyhow::Result;

  use super::*;
  use crate::frames::RSocketFrame;

  static BYTES: &[u8] = include_bytes!("../../testdata/frame.payload.bin");

  #[test]
  fn test_decode() -> Result<()> {
    println!("RAW: {:?}", BYTES);
    let p = PayloadFrame::decode_all(BYTES.into())?;
    assert_eq!(p.stream_id, 1234);
    assert_eq!(p.data, Bytes::from("hello"));
    assert_eq!(p.metadata, Bytes::from("hello"));
    Ok(())
  }

  #[test]
  fn test_encode() -> Result<()> {
    let payload = PayloadFrame {
      stream_id: 1234,
      metadata: Bytes::from("hello"),
      data: Bytes::from("hello"),
      follows: true,
      complete: true,
      next: true,
    };
    let encoded = payload.encode();
    assert_eq!(encoded, Bytes::from(BYTES));
    Ok(())
  }
}
