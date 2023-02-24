//! Library-specific error types and utility functions

use crate::frames::ErrorCode;

/// Error type for wasmRS RSocket errors.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub enum Error {
  /// An error associated with OperationList methods.
  OpList(String),
  /// A generic RSocket error.
  RSocket(u32),
  /// Used when the receiver for a [crate::WasmSocket] has already been taken.
  ReceiverAlreadyGone,
  /// Variant used when a frame is treated as the wrong type.
  WrongType,
  /// Could not convert string from passed bytes.
  StringConversion,
  /// [crate::Metadata] not found in [crate::Payload]
  MetadataNotFound,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::RSocket(code) => f.write_str((Into::<u32>::into(*code)).to_string().as_str()),
      Error::OpList(msg) => f.write_str(msg),
      Error::ReceiverAlreadyGone => f.write_str("Received already taken"),
      Error::WrongType => f.write_str("Tried to decode frame with wrong frame decoder"),
      Error::StringConversion => f.write_str("Could not read string bytes"),
      Error::MetadataNotFound => f.write_str("No metadata found"),
    }
  }
}

#[derive(Debug)]
#[must_use]
/// The error type used for all [wasmrs_rx::Mono]/[wasmrs_rx::Flux] payloads.
pub struct PayloadError {
  /// The error code.
  pub code: u32,
  /// The error message.
  pub msg: String,
}

impl PayloadError {
  /// Create a new [PayloadError] with the passed code and message.
  pub fn new(code: u32, msg: impl AsRef<str>) -> Self {
    Self {
      code,
      msg: msg.as_ref().to_owned(),
    }
  }

  /// Create a new [PayloadError] with the [ErrorCode::ApplicationError] code.
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
