//! Library-specific error types and utility functions

use crate::{ErrorCode, FrameType};

/// Error type for wasmRS RSocket errors.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub enum Error {
    RSocket(u32),
    SendFailed(u8),
    RecvFailed(u8),
    ReceiverAlreadyGone,
    RxMissing,
    WrongType(FrameType, FrameType),
    StringConversion,
    MetadataNotFound,
    StringDecode,
    RequestCancelled(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RSocket(code) => f.write_str((Into::<u32>::into(*code)).to_string().as_str()),
            _ => f.write_str("Error"),
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct PayloadError {
    pub code: u32,
    pub msg: String,
}

impl PayloadError {
    pub fn new(code: u32, msg: impl AsRef<str>) -> Self {
        Self {
            code,
            msg: msg.as_ref().to_owned(),
        }
    }

    pub fn application_error(msg: impl AsRef<str>) -> Self {
        Self {
            code: ErrorCode::ApplicationError.into(),
            msg: msg.as_ref().to_owned(),
        }
    }
}
impl std::error::Error for PayloadError {}
impl std::fmt::Display for PayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl From<wasmrs_codec::error::Error> for PayloadError {
    fn from(e: wasmrs_codec::error::Error) -> Self {
        PayloadError::application_error(e.to_string())
    }
}
