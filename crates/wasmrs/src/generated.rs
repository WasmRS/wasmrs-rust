/************************************************
 * THIS FILE IS GENERATED, DO NOT EDIT          *
 *                                              *
 * See https://apexlang.io for more information *
 ***********************************************/
#![allow(
    unused_qualifications,
    missing_copy_implementations,
    clippy::from_over_into,
    missing_debug_implementations
)]

pub type FrameFlags = u16;

#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
#[derive()]
pub struct RequestChannel(pub RequestPayload);

#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
#[derive()]
pub struct RequestStream(pub RequestPayload);

#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
#[derive()]
pub struct RequestResponse(pub RequestPayload);

#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
#[derive()]
pub struct RequestFnF(pub RequestPayload);

/// Six (6) bytes reserved for FrameHeader information.
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct FrameHeader {
    pub header: bytes::Bytes,
}
#[derive(Clone, Default)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct Payload {
    pub metadata: Option<bytes::Bytes>,
    pub data: Option<bytes::Bytes>,
}
/// A Payload frame.
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct PayloadFrame {
    /// The stream ID this frame belongs to.
    pub stream_id: u32,
    /// Any metadata associated with the Payload as raw bytes.
    pub metadata: bytes::Bytes,
    /// The actual payload data as raw bytes.
    pub data: bytes::Bytes,
    /// Whether this payload is broken up into multiple frames.
    pub follows: bool,
    /// Whether or not this frame is the last frame in a stream.
    pub complete: bool,
    /// TODO
    pub next: bool,
}
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct Cancel {
    /// The stream ID this frame belongs to.
    pub stream_id: u32,
}
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct ErrorFrame {
    /// The stream ID this frame belongs to.
    pub stream_id: u32,
    pub code: u32,
    pub data: String,
}
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct RequestN {
    /// The stream ID this frame belongs to.
    pub stream_id: u32,
    pub n: u32,
}
#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct RequestPayload {
    /// The type of Request this payload creates.
    pub frame_type: FrameType,
    /// The stream ID this frame belongs to.
    pub stream_id: u32,
    /// Any metadata associated with the Payload as raw bytes.
    pub metadata: bytes::Bytes,
    /// The actual payload data as raw bytes.
    pub data: bytes::Bytes,
    /// Whether this payload is broken up into multiple frames.
    pub follows: bool,
    /// Whether or not this frame is the last frame in a stream.
    pub complete: bool,
    /// TODO
    pub initial_n: u32,
}
/// Metadata associated with the frame.
#[derive(Clone)]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[must_use]
pub struct Metadata {
    /// The operation index.
    pub index: u32,
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
impl std::convert::TryFrom<u8> for FrameType {
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

#[derive()]
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
impl std::convert::TryFrom<u32> for FrameFlag {
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
impl std::convert::TryFrom<u32> for ErrorCode {
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
