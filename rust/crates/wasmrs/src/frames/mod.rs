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

use crate::util::from_u32_bytes;
use crate::Error;

use self::f_cancel::Cancel;
use self::f_error::ErrorFrame;
use self::f_payload::PayloadFrame;
use self::f_request_channel::RequestChannel;
use self::f_request_fnf::RequestFnF;
use self::f_request_n::RequestN;
use self::f_request_response::RequestResponse;
use self::f_request_stream::RequestStream;

pub type FrameFlags = u16;

/// Six (6) bytes reserved for FrameHeader information.
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct FrameHeader {
  pub header: Bytes,
}
#[derive(Clone, Default)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct Payload {
  pub metadata: Option<Bytes>,
  pub data: Option<Bytes>,
}

/// Metadata associated with the frame.
#[derive(Clone, Copy)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct Metadata {
  /// The operation index.
  pub index: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FrameType {
  Reserved,
  Setup,
  Lease,
  Keepalive,
  RequestResponse,
  RequestFnf,
  RequestStream,
  RequestChannel,
  RequestN,
  Cancel,
  Payload,
  Err,
  MetadataPush,
  Resume,
  ResumeOk,
  Ext,
}
impl std::fmt::Display for FrameType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Reserved => "RESERVED",
        Self::Setup => "SETUP",
        Self::Lease => "LEASE",
        Self::Keepalive => "KEEPALIVE",
        Self::RequestResponse => "REQUEST_RESPONSE",
        Self::RequestFnf => "REQUEST_FNF",
        Self::RequestStream => "REQUEST_STREAM",
        Self::RequestChannel => "REQUEST_CHANNEL",
        Self::RequestN => "REQUEST_N",
        Self::Cancel => "CANCEL",
        Self::Payload => "PAYLOAD",
        Self::Err => "ERROR",
        Self::MetadataPush => "METADATA_PUSH",
        Self::Resume => "RESUME",
        Self::ResumeOk => "RESUME_OK",
        Self::Ext => "EXT",
      }
    )
  }
}
impl TryFrom<u8> for FrameType {
  type Error = String;
  fn try_from(index: u8) -> Result<Self, Self::Error> {
    match index {
      0 => Ok(Self::Reserved),
      1 => Ok(Self::Setup),
      2 => Ok(Self::Lease),
      3 => Ok(Self::Keepalive),
      4 => Ok(Self::RequestResponse),
      5 => Ok(Self::RequestFnf),
      6 => Ok(Self::RequestStream),
      7 => Ok(Self::RequestChannel),
      8 => Ok(Self::RequestN),
      9 => Ok(Self::Cancel),
      10 => Ok(Self::Payload),
      11 => Ok(Self::Err),
      12 => Ok(Self::MetadataPush),
      13 => Ok(Self::Resume),
      14 => Ok(Self::ResumeOk),
      63 => Ok(Self::Ext),
      _ => Err(format!("{} is not a valid index for FrameType", index)),
    }
  }
}
impl Into<u32> for FrameType {
  fn into(self) -> u32 {
    match self {
      Self::Reserved => unreachable!(),
      Self::Setup => 1,
      Self::Lease => 2,
      Self::Keepalive => 3,
      Self::RequestResponse => 4,
      Self::RequestFnf => 5,
      Self::RequestStream => 6,
      Self::RequestChannel => 7,
      Self::RequestN => 8,
      Self::Cancel => 9,
      Self::Payload => 10,
      Self::Err => 11,
      Self::MetadataPush => 12,
      Self::Resume => 13,
      Self::ResumeOk => 14,
      Self::Ext => 63,
    }
  }
}

#[derive(Clone, Copy)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub enum FrameFlag {
  Metadata,
  Follows,
  Complete,
  Next,
  Ignore,
}
impl std::fmt::Display for FrameFlag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Metadata => "M",
        Self::Follows => "FRS",
        Self::Complete => "CL",
        Self::Next => "N",
        Self::Ignore => "I",
      }
    )
  }
}
impl TryFrom<u32> for FrameFlag {
  type Error = String;
  fn try_from(index: u32) -> Result<Self, Self::Error> {
    match index {
      0 => Ok(Self::Metadata),
      1 => Ok(Self::Follows),
      2 => Ok(Self::Complete),
      3 => Ok(Self::Next),
      4 => Ok(Self::Ignore),
      _ => Err(format!("{} is not a valid index for FrameFlag", index)),
    }
  }
}
impl Into<u32> for FrameFlag {
  fn into(self) -> u32 {
    match self {
      Self::Metadata => unreachable!(),
      Self::Follows => 1,
      Self::Complete => 2,
      Self::Next => 3,
      Self::Ignore => 4,
    }
  }
}

/// RSocket Error Codes
#[derive(Copy, Clone)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub enum ErrorCode {
  InvalidSetup,
  UnsupportedSetup,
  RejectedSetup,
  RejectedResume,
  ConnectionError,
  ConnectionClose,
  ApplicationError,
  Rejected,
  Canceled,
  Invalid,
  Reserved,
}
impl TryFrom<u32> for ErrorCode {
  type Error = String;
  fn try_from(index: u32) -> Result<Self, Self::Error> {
    match index {
      1 => Ok(Self::InvalidSetup),
      2 => Ok(Self::UnsupportedSetup),
      3 => Ok(Self::RejectedSetup),
      4 => Ok(Self::RejectedResume),
      257 => Ok(Self::ConnectionError),
      258 => Ok(Self::ConnectionClose),
      513 => Ok(Self::ApplicationError),
      514 => Ok(Self::Rejected),
      515 => Ok(Self::Canceled),
      516 => Ok(Self::Invalid),
      4294967295 => Ok(Self::Reserved),
      _ => Err(format!("{} is not a valid index for ErrorCode", index)),
    }
  }
}
impl Into<u32> for ErrorCode {
  fn into(self) -> u32 {
    match self {
      Self::InvalidSetup => 1,
      Self::UnsupportedSetup => 2,
      Self::RejectedSetup => 3,
      Self::RejectedResume => 4,
      Self::ConnectionError => 257,
      Self::ConnectionClose => 258,
      Self::ApplicationError => 513,
      Self::Rejected => 514,
      Self::Canceled => 515,
      Self::Invalid => 516,
      Self::Reserved => 4294967295,
    }
  }
}

#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub enum Frame {
  PayloadFrame(PayloadFrame),
  Cancel(Cancel),
  ErrorFrame(ErrorFrame),
  RequestN(RequestN),
  RequestResponse(RequestResponse),
  RequestFnF(RequestFnF),
  RequestStream(RequestStream),
  RequestChannel(RequestChannel),
}

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
    let stream_id = header.stream_id();
    Self::_decode(header, bytes).map_err(|e| (stream_id, e))
  }

  pub(crate) fn _decode(header: FrameHeader, buffer: Bytes) -> Result<Frame, Error> {
    let frame = match header.frame_type() {
      FrameType::Reserved => todo!(),
      FrameType::Setup => todo!(),
      FrameType::RequestResponse => {
        Frame::RequestResponse(f_request_response::RequestResponse::decode_frame(&header, buffer)?)
      }
      FrameType::RequestFnf => Frame::RequestFnF(f_request_fnf::RequestFnF::decode_frame(&header, buffer)?),
      FrameType::RequestStream => Frame::RequestStream(f_request_stream::RequestStream::decode_frame(&header, buffer)?),
      FrameType::RequestChannel => {
        Frame::RequestChannel(f_request_channel::RequestChannel::decode_frame(&header, buffer)?)
      }
      FrameType::RequestN => Frame::RequestN(f_request_n::RequestN::decode_frame(&header, buffer)?),
      FrameType::Cancel => Frame::Cancel(Cancel {
        stream_id: header.stream_id(),
      }),
      FrameType::Payload => Frame::PayloadFrame(f_payload::PayloadFrame::decode_frame(&header, buffer)?),
      FrameType::Err => Frame::ErrorFrame(f_error::ErrorFrame::decode_frame(&header, buffer)?),
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
