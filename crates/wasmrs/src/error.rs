//! Library-specific error types and utility functions

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
  /// Error encoding or decoding an RSocket frame
  Frame(wasmrs_frames::Error),
  /// Error associate with recording frames during debug
  Record(String),
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
      Error::Frame(e) => f.write_str(e.to_string().as_str()),
      Error::Record(e) => f.write_str(e.as_str()),
    }
  }
}

impl From<wasmrs_frames::Error> for Error {
  fn from(value: wasmrs_frames::Error) -> Self {
    Error::Frame(value)
  }
}
