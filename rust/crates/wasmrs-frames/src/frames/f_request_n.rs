use bytes::Bytes;

use super::{Error, FrameHeader, FrameType, RSocketFrame};
use crate::util::from_u32_bytes;
use crate::Frame;

#[derive(Clone)]
#[allow(missing_copy_implementations)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct RequestN {
  /// The stream ID this frame belongs to.
  pub stream_id: u32,
  pub n: u32,
}

impl RequestN {}

impl RSocketFrame<RequestN> for RequestN {
  const FRAME_TYPE: FrameType = FrameType::RequestN;

  fn stream_id(&self) -> u32 {
    self.stream_id
  }

  fn decode_all(mut buffer: Bytes) -> Result<Self, Error> {
    let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
    Self::decode_frame(&header, buffer)
  }

  fn decode_frame(header: &FrameHeader, mut buffer: Bytes) -> Result<Self, Error> {
    Self::check_type(header)?;
    Ok(RequestN {
      stream_id: header.stream_id(),
      n: from_u32_bytes(&buffer.split_to(4)),
    })
  }

  fn encode(self) -> Bytes {
    [self.gen_header().encode(), self.n.to_be_bytes().to_vec().into()]
      .concat()
      .into()
  }

  fn gen_header(&self) -> FrameHeader {
    FrameHeader::new(self.stream_id, FrameType::RequestN, 0)
  }
}

#[cfg(test)]
mod test {
  use anyhow::Result;

  use super::*;
  use crate::frames::RSocketFrame;

  static BYTES: &[u8] = include_bytes!("../../testdata/frame.request_n.bin");

  #[test]
  fn test_decode() -> Result<()> {
    println!("RAW {:?}", BYTES);
    let p = RequestN::decode_all(BYTES.into())?;
    assert_eq!(p.stream_id, 1234);
    Ok(())
  }

  #[test]
  fn test_encode() -> Result<()> {
    let payload = RequestN { stream_id: 1234, n: 15 };
    let encoded = payload.encode();
    assert_eq!(encoded, Bytes::from(BYTES));
    Ok(())
  }
}
