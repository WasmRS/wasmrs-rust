#[derive(Debug)]
/// The error variants used by wasmrs-guest.
pub enum Error {
  /// No handler could be found for the passed index or namespace + operation.
  NoHandler,
  /// The handler failed.
  HandlerFail(String),
  /// Error reading frame buffer.
  BufferRead,
  /// Internal Error.
  Internal(wasmrs::Error),
  /// Error decoding payload or metadata.
  Codec(String),
  /// Error in the asynchronous runtime.
  Runtime(String),
}

impl From<wasmrs::Error> for Error {
  fn from(e: wasmrs::Error) -> Self {
    Self::Internal(e)
  }
}

impl From<wasmrs_codec::error::Error> for Error {
  fn from(e: wasmrs_codec::error::Error) -> Self {
    Self::Codec(e.to_string())
  }
}

impl From<wasmrs_runtime::Error> for Error {
  fn from(e: wasmrs_runtime::Error) -> Self {
    Self::Runtime(e.to_string())
  }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::NoHandler => f.write_str("No handler found"),
      Error::HandlerFail(msg) => f.write_str(msg),
      Error::BufferRead => f.write_str("Error reading buffer"),
      Error::Internal(e) => f.write_str(&e.to_string()),
      Error::Codec(e) => f.write_str(e),
      Error::Runtime(e) => f.write_str(e),
    }
  }
}
impl From<std::io::Error> for Error {
  fn from(_: std::io::Error) -> Self {
    Error::BufferRead
  }
}

impl From<wasmrs_frames::Error> for Error {
  fn from(value: wasmrs_frames::Error) -> Self {
    Error::Internal(value.into())
  }
}
