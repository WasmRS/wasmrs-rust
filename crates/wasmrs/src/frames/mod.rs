pub(crate) mod f_cancel;
pub(crate) mod f_error;
pub(crate) mod f_payload;
pub(crate) mod f_request_channel;
pub(crate) mod f_request_fnf;
pub(crate) mod f_request_n;
pub(crate) mod f_request_response;
pub(crate) mod f_request_stream;
pub(crate) mod header;
pub(crate) mod metadata;
pub(crate) mod request_payload;

use bytes::Bytes;

use crate::generated::{
  self as frames,
  Cancel,
  ErrorFrame,
  FrameFlags,
  FrameHeader,
  FrameType,
  Payload,
  PayloadFrame,
  RequestChannel,
  RequestResponse,
  RequestStream,
};
use crate::util::from_u32_bytes;
use crate::{Error, Frame, Metadata, RequestFnF, RequestN};

impl crate::generated::FrameFlag {}

impl Payload {
  pub fn new(metadata: Bytes, data: Bytes) -> Self {
    Self {
      metadata: Some(metadata),
      data: Some(data),
    }
  }
  pub fn new_optional(metadata: Option<Bytes>, data: Option<Bytes>) -> Self {
    Self { metadata, data }
  }
  pub fn empty() -> Self {
    Self {
      metadata: None,
      data: None,
    }
  }
  pub fn parse_metadata(&self) -> Result<Metadata, Error> {
    if self.metadata.is_none() {
      return Err(Error::MetadataNotFound);
    }
    let bytes = self.metadata.as_ref().unwrap();
    let index = from_u32_bytes(&bytes[0..4]);

    Ok(Metadata { index })
  }
}

impl From<Frame> for Result<Option<Payload>, crate::PayloadError> {
  fn from(frame: Frame) -> Self {
    match frame {
      Frame::PayloadFrame(frame) => Ok(Some(Payload::new(frame.metadata, frame.data))),
      Frame::Cancel(_frame) => todo!(),
      Frame::ErrorFrame(frame) => Err(crate::PayloadError::new(frame.code, frame.data)),
      Frame::RequestN(_frame) => todo!(),
      Frame::RequestResponse(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
      Frame::RequestFnF(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
      Frame::RequestStream(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
      Frame::RequestChannel(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
    }
  }
}

impl Frame {
  pub(crate) const LEN_HEADER: usize = 6;
  pub(crate) const FLAG_IGNORE: FrameFlags = 1 << 4;
  pub(crate) const FLAG_NEXT: FrameFlags = 1 << 5;
  pub(crate) const FLAG_COMPLETE: FrameFlags = 1 << 6;
  pub(crate) const FLAG_FOLLOW: FrameFlags = 1 << 7;
  pub(crate) const FLAG_METADATA: FrameFlags = 1 << 8;
  pub const REQUEST_MAX: u32 = 0x7FFF_FFFF; // 2147483647

  pub fn is_followable_or_payload(&self) -> (bool, bool) {
    match &self {
      Frame::RequestFnF(_) => (true, false),
      Frame::RequestResponse(_) => (true, false),
      Frame::RequestStream(_) => (true, false),
      Frame::RequestChannel(_) => (true, false),
      Frame::PayloadFrame(_) => (true, true),
      _ => (false, false),
    }
  }

  #[must_use]
  pub fn stream_id(&self) -> u32 {
    match self {
      Frame::PayloadFrame(frame) => frame.stream_id,
      Frame::Cancel(frame) => frame.stream_id,
      Frame::ErrorFrame(frame) => frame.stream_id,
      Frame::RequestN(frame) => frame.stream_id,
      Frame::RequestResponse(frame) => frame.0.stream_id,
      Frame::RequestFnF(frame) => frame.0.stream_id,
      Frame::RequestStream(frame) => frame.0.stream_id,
      Frame::RequestChannel(frame) => frame.0.stream_id,
    }
  }

  #[must_use]
  pub fn get_flag(&self) -> FrameFlags {
    match self {
      Frame::PayloadFrame(frame) => frame.get_flag(),
      Frame::Cancel(frame) => frame.get_flag(),
      Frame::ErrorFrame(frame) => frame.get_flag(),
      Frame::RequestN(frame) => frame.get_flag(),
      Frame::RequestResponse(frame) => frame.get_flag(),
      Frame::RequestFnF(frame) => frame.get_flag(),
      Frame::RequestStream(frame) => frame.get_flag(),
      Frame::RequestChannel(frame) => frame.get_flag(),
    }
  }

  #[must_use]
  pub fn frame_type(&self) -> FrameType {
    match self {
      Frame::PayloadFrame(_) => FrameType::Payload,
      Frame::Cancel(_) => FrameType::Cancel,
      Frame::ErrorFrame(_) => FrameType::Err,
      Frame::RequestN(_) => FrameType::RequestN,
      Frame::RequestResponse(_) => FrameType::RequestResponse,
      Frame::RequestFnF(_) => FrameType::RequestFnf,
      Frame::RequestStream(_) => FrameType::RequestStream,
      Frame::RequestChannel(_) => FrameType::RequestChannel,
    }
  }

  pub fn decode(mut bytes: Bytes) -> Result<Frame, (u32, Error)> {
    let header = FrameHeader::from_bytes(bytes.split_to(Frame::LEN_HEADER));
    println!("{}", header);
    let stream_id = header.stream_id();
    Self::_decode(header, bytes).map_err(|e| (stream_id, e))
  }

  pub(crate) fn _decode(header: FrameHeader, buffer: Bytes) -> Result<Frame, Error> {
    let frame = match header.frame_type() {
      FrameType::Reserved => todo!(),
      FrameType::Setup => todo!(),
      FrameType::RequestResponse => {
        frames::Frame::RequestResponse(frames::RequestResponse::decode_frame(&header, buffer)?)
      }
      FrameType::RequestFnf => frames::Frame::RequestFnF(frames::RequestFnF::decode_frame(&header, buffer)?),
      FrameType::RequestStream => frames::Frame::RequestStream(frames::RequestStream::decode_frame(&header, buffer)?),
      FrameType::RequestChannel => {
        frames::Frame::RequestChannel(frames::RequestChannel::decode_frame(&header, buffer)?)
      }
      FrameType::RequestN => frames::Frame::RequestN(RequestN::decode_frame(&header, buffer)?),
      FrameType::Cancel => frames::Frame::Cancel(Cancel {
        stream_id: header.stream_id(),
      }),
      FrameType::Payload => frames::Frame::PayloadFrame(frames::PayloadFrame::decode_frame(&header, buffer)?),
      FrameType::Err => frames::Frame::ErrorFrame(frames::ErrorFrame::decode_frame(&header, buffer)?),
      FrameType::Ext => todo!(),
      _ => unreachable!(), // Maybe not todo?,
    };
    Ok(frame)
  }

  #[must_use]
  pub fn encode(self) -> Bytes {
    match self {
      Frame::PayloadFrame(f) => f.encode(),
      Frame::Cancel(f) => f.encode(),
      Frame::ErrorFrame(f) => f.encode(),
      Frame::RequestN(f) => f.encode(),
      Frame::RequestResponse(f) => f.encode(),
      Frame::RequestFnF(f) => f.encode(),
      Frame::RequestStream(f) => f.encode(),
      Frame::RequestChannel(f) => f.encode(),
    }
  }

  pub fn new_error(stream_id: u32, code: u32, data: impl AsRef<str>) -> Frame {
    Frame::ErrorFrame(ErrorFrame {
      stream_id,
      code,
      data: data.as_ref().to_owned(),
    })
  }

  pub fn new_cancel(stream_id: u32) -> Frame {
    Frame::Cancel(Cancel { stream_id })
  }

  pub fn new_request_n(stream_id: u32, n: u32, _flags: FrameFlags) -> Frame {
    Frame::RequestN(RequestN { stream_id, n })
  }

  pub fn new_request_response(stream_id: u32, payload: Payload, flags: FrameFlags) -> Frame {
    Frame::RequestResponse(RequestResponse::from_payload(stream_id, payload, flags, 0))
  }

  pub fn new_request_stream(stream_id: u32, payload: Payload, flags: FrameFlags) -> Frame {
    Frame::RequestStream(RequestStream::from_payload(stream_id, payload, flags, 0))
  }

  pub fn new_request_channel(stream_id: u32, payload: Payload, flags: FrameFlags, initial_n: u32) -> Frame {
    Frame::RequestChannel(RequestChannel::from_payload(stream_id, payload, flags, initial_n))
  }

  pub fn new_request_fnf(stream_id: u32, payload: Payload, flags: FrameFlags) -> Frame {
    Frame::RequestFnF(RequestFnF::from_payload(stream_id, payload, flags, 0))
  }

  pub fn new_payload(stream_id: u32, payload: Payload, flags: FrameFlags) -> Frame {
    Frame::PayloadFrame(PayloadFrame::from_payload(stream_id, payload, flags))
  }
}

pub trait RSocketFrame<T> {
  const FRAME_TYPE: FrameType;
  fn check_type(header: &FrameHeader) -> Result<(), Error> {
    if header.frame_type() == Self::FRAME_TYPE {
      Ok(())
    } else {
      Err(Error::WrongType)
    }
  }
  fn kind(&self) -> FrameType {
    Self::FRAME_TYPE
  }
  fn stream_id(&self) -> u32;
  fn decode_all(buffer: Bytes) -> Result<T, Error>;
  fn decode_frame(header: &FrameHeader, buffer: Bytes) -> Result<T, Error>;
  fn encode(self) -> Bytes;
  fn gen_header(&self) -> FrameHeader;
  fn get_flag(&self) -> FrameFlags {
    0
  }
}

pub(crate) trait RSocketFlags {
  fn flag_next(&self) -> bool;
  fn flag_metadata(&self) -> bool;
  fn flag_complete(&self) -> bool;
  fn flag_follows(&self) -> bool;
  fn flag_ignore(&self) -> bool;
}

impl RSocketFlags for FrameFlags {
  fn flag_next(&self) -> bool {
    self & Frame::FLAG_NEXT == Frame::FLAG_NEXT
  }

  fn flag_metadata(&self) -> bool {
    self & Frame::FLAG_METADATA == Frame::FLAG_METADATA
  }

  fn flag_complete(&self) -> bool {
    self & Frame::FLAG_COMPLETE == Frame::FLAG_COMPLETE
  }

  fn flag_follows(&self) -> bool {
    self & Frame::FLAG_FOLLOW == Frame::FLAG_FOLLOW
  }

  fn flag_ignore(&self) -> bool {
    self & Frame::FLAG_IGNORE == Frame::FLAG_IGNORE
  }
}
