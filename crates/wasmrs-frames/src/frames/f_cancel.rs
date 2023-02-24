use bytes::Bytes;

use super::{Error, FrameHeader, FrameType, RSocketFrame};
use crate::Frame;

#[derive(Clone)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct Cancel {
  /// The stream ID this frame belongs to.
  pub stream_id: u32,
}

impl RSocketFrame<Cancel> for Cancel {
  const FRAME_TYPE: FrameType = FrameType::Cancel;

  fn stream_id(&self) -> u32 {
    self.stream_id
  }

  fn decode_all(mut buffer: Bytes) -> Result<Self, Error> {
    let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
    Self::decode_frame(&header, buffer)
  }

  fn decode_frame(header: &FrameHeader, _buffer: Bytes) -> Result<Self, Error> {
    Self::check_type(header)?;
    Ok(Cancel {
      stream_id: header.stream_id(),
    })
  }

  fn encode(self) -> Bytes {
    self.gen_header().encode()
  }

  fn gen_header(&self) -> FrameHeader {
    FrameHeader::new(self.stream_id, FrameType::Cancel, 0)
  }
}

#[cfg(test)]
mod test {
  use anyhow::Result;

  use super::*;
  use crate::frames::RSocketFrame;

  static BYTES: &[u8] = include_bytes!("../../testdata/frame.cancel.bin");

  #[test]
  fn test_decode() -> Result<()> {
    println!("RAW: {:?}", BYTES);
    let p = Cancel::decode_all(BYTES.into())?;
    assert_eq!(p.stream_id, 1234);
    Ok(())
  }

  #[test]
  fn test_encode() -> Result<()> {
    let payload = Cancel { stream_id: 1234 };
    let encoded = payload.encode();
    assert_eq!(encoded, Bytes::from(BYTES));
    Ok(())
  }
}
