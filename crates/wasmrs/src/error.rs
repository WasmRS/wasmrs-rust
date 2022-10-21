//! Library-specific error types and utility functions

use crate::{ErrorCode, FrameType};

/// Error type for wasmRS RSocket errors.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub enum Error {
  OpList(String),
  RSocket(u32),
  SendFailed(u8),
  RecvFailed(u8),
  ReceiverAlreadyGone,
  WrongType,
  StringConversion,
  MetadataNotFound,
  RequestCancelled(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::RSocket(code) => f.write_str((Into::<u32>::into(*code)).to_string().as_str()),
      Error::OpList(msg) => f.write_str(msg),
      Error::SendFailed(_) => f.write_str("Send failed"),
      Error::RecvFailed(_) => f.write_str("Receive failed"),
      Error::ReceiverAlreadyGone => f.write_str("Received already taken"),
      Error::WrongType => f.write_str("Tried to decode frame with wrong frame decoder"),
      Error::StringConversion => f.write_str("Could not read string bytes"),
      Error::MetadataNotFound => f.write_str("No metadata found"),
      Error::RequestCancelled(msg) => f.write_str(msg),
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
    app_err(&e)
  }
}

impl From<tokio::sync::oneshot::error::RecvError> for PayloadError {
  fn from(e: tokio::sync::oneshot::error::RecvError) -> Self {
    app_err(&e)
  }
}

impl From<Error> for PayloadError {
  fn from(e: Error) -> Self {
    app_err(&e)
  }
}

impl From<Box<dyn std::error::Error>> for PayloadError {
  fn from(e: Box<dyn std::error::Error>) -> Self {
    app_err(e.as_ref())
  }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for PayloadError {
  fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
    app_err(e.as_ref())
  }
}

fn app_err(e: &dyn std::error::Error) -> PayloadError {
  PayloadError::application_error(e.to_string())
}
