use bytes::{BufMut, Bytes, BytesMut};

use super::{Error, FrameHeader, FrameType, RSocketFrame};
use crate::util::{from_u24_bytes, from_u32_bytes, to_u24_bytes};
use crate::{Frame, FrameFlags};

#[derive(Clone)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct ErrorFrame {
  /// The stream ID this frame belongs to.
  pub stream_id: u32,
  /// The error code.
  pub code: u32,
  /// Any metadata associated with the Error as raw bytes.
  pub metadata: Option<Bytes>,
  /// The error message data.
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
    let metadata = if header.has_metadata() {
      let metadata_len = from_u24_bytes(&buffer.split_to(3)) as usize;
      Some(buffer.split_to(metadata_len))
    } else {
      None
    };

    Ok(ErrorFrame {
      stream_id: header.stream_id(),
      metadata,
      code: from_u32_bytes(&buffer.split_to(4)),
      data: String::from_utf8(buffer.to_vec()).map_err(|_| crate::Error::StringConversion)?,
    })
  }

  fn encode(self) -> Bytes {
    let header = self.gen_header().encode();
    let (mlen, md) = self.metadata.map_or_else(
      || (Bytes::new(), Bytes::new()),
      |md| (to_u24_bytes(md.len() as u32), md),
    );

    let code = self.code.to_be_bytes();
    let data = self.data.into_bytes();
    let mut bytes = BytesMut::with_capacity(Frame::LEN_HEADER + code.len() + data.len());
    bytes.put(header);
    bytes.put(mlen);
    bytes.put(md);
    bytes.put(code.as_slice());
    bytes.put(data.as_slice());
    bytes.freeze()
  }

  fn gen_header(&self) -> FrameHeader {
    FrameHeader::new(self.stream_id, FrameType::Err, self.get_flag())
  }

  fn get_flag(&self) -> FrameFlags {
    let mut flags = 0;
    if self.metadata.is_some() {
      flags |= Frame::FLAG_METADATA;
    }
    flags
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
      metadata: None,
      data: "errstr".to_owned(),
      code: 11,
    };
    let encoded = payload.encode();
    assert_eq!(encoded, Bytes::from(BYTES));
    Ok(())
  }
}
