use bytes::{BufMut, Bytes, BytesMut};

use super::{Error, FrameHeader, FrameType, RSocketFrame};
use crate::util::from_u32_bytes;
use crate::Frame;

#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[cfg_attr(feature = "serde", derive(Clone))]
#[must_use]
pub struct ErrorFrame {
  /// The stream ID this frame belongs to.
  pub stream_id: u32,
  pub code: u32,
  pub data: String,
}

impl ErrorFrame {}

impl RSocketFrame<ErrorFrame> for ErrorFrame {
  const FRAME_TYPE: FrameType = FrameType::Err;

  fn stream_id(&self) -> u32 {
    self.stream_id
  }

  fn decode_all(mut buffer: Bytes) -> Result<Self, Error> {
    let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
    Self::decode_frame(&header, buffer)
  }

  fn decode_frame(header: &FrameHeader, mut buffer: Bytes) -> Result<Self, Error> {
    Self::check_type(header)?;

    Ok(ErrorFrame {
      stream_id: header.stream_id(),
      code: from_u32_bytes(&buffer.split_to(4)),
      data: String::from_utf8(buffer.to_vec()).map_err(|_| crate::Error::StringConversion)?,
    })
  }

  fn encode(self) -> Bytes {
    let header = self.gen_header().encode();
    let code = self.code.to_be_bytes();
    let data = self.data.into_bytes();
    let mut bytes = BytesMut::with_capacity(Frame::LEN_HEADER + code.len() + data.len());
    bytes.put(header);
    bytes.put(code.as_slice());
    bytes.put(data.as_slice());
    bytes.freeze()
  }

  fn gen_header(&self) -> FrameHeader {
    FrameHeader::new(self.stream_id, FrameType::Err, 0)
  }
}

#[cfg(test)]
mod test {
  use anyhow::Result;

  use super::*;
  use crate::frames::RSocketFrame;

  static BYTES: &[u8] = include_bytes!("../../testdata/frame.error.bin");

  #[test]
  fn test_decode() -> Result<()> {
    println!("{:?}", BYTES);
    let p = ErrorFrame::decode_all(BYTES.into())?;
    assert_eq!(p.stream_id, 1234);
    assert_eq!(&p.data, "errstr");
    assert_eq!(p.code, 11);
    Ok(())
  }

  #[test]
  fn test_encode() -> Result<()> {
    let payload = ErrorFrame {
      stream_id: 1234,
      data: "errstr".to_owned(),
      code: 11,
    };
    let encoded = payload.encode();
    assert_eq!(encoded, Bytes::from(BYTES));
    Ok(())
  }
}
