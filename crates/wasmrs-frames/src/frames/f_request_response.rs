use bytes::Bytes;

use super::{request_payload::RequestPayload, Error, FrameFlags, FrameHeader, FrameType, RSocketFrame};
use crate::{Frame, RawPayload};

#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
#[derive(Clone)]
pub struct RequestResponse(pub RequestPayload);

impl RequestResponse {
  pub(crate) fn from_payload(stream_id: u32, payload: RawPayload, flags: FrameFlags, initial_n: u32) -> Self {
    Self(RequestPayload::from_payload(
      stream_id,
      payload,
      Self::FRAME_TYPE,
      flags,
      initial_n,
    ))
  }
}

impl RSocketFrame<RequestResponse> for RequestResponse {
  const FRAME_TYPE: FrameType = FrameType::RequestResponse;

  fn stream_id(&self) -> u32 {
    self.0.stream_id
  }

  fn decode_all(mut buffer: Bytes) -> Result<Self, Error> {
    let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
    Self::decode_frame(&header, buffer)
  }

  fn decode_frame(header: &FrameHeader, buffer: Bytes) -> Result<Self, Error> {
    Self::check_type(header)?;
    Ok(Self(RequestPayload::decode(header, buffer)?))
  }

  fn encode(self) -> Bytes {
    self.0.encode()
  }

  fn gen_header(&self) -> FrameHeader {
    FrameHeader::new(self.0.stream_id, FrameType::RequestResponse, Frame::FLAG_METADATA)
  }

  fn get_flag(&self) -> FrameFlags {
    self.0.get_flags()
  }
}

impl From<RequestResponse> for RawPayload {
  fn from(req: RequestResponse) -> Self {
    req.0.into()
  }
}

#[cfg(test)]
mod test {
  use anyhow::Result;

  use super::*;

  static BYTES: &[u8] = include_bytes!("../../testdata/frame.request_response.bin");

  #[test]
  fn test_decode() -> Result<()> {
    println!("RAW: {:?}", BYTES);
    let p = RequestResponse::decode_all(BYTES.into())?;
    assert_eq!(p.0.stream_id, 1234);
    assert_eq!(p.0.data, Bytes::from(b"hello".as_slice()));
    Ok(())
  }

  #[test]
  fn test_encode() -> Result<()> {
    let payload = RequestPayload {
      frame_type: FrameType::RequestResponse,
      stream_id: 1234,
      metadata: Bytes::from("hello"),
      data: Bytes::from("hello"),
      follows: true,
      complete: true,
      initial_n: 1,
    };
    let this = RequestResponse(payload);
    let encoded = this.encode();
    assert_eq!(encoded, Bytes::from(BYTES));
    Ok(())
  }
}
