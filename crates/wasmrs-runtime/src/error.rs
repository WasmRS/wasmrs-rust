//! Library-specific error types and utility functions

/// Error type for wasmRS Runtime errors.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub enum Error {
  /// Sending on the channel failed.
  SendFailed(u8),
  /// Receiving from the channel failed.
  RecvFailed(u8),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::SendFailed(_) => f.write_str("Send failed"),
      Error::RecvFailed(_) => f.write_str("Receive failed"),
    }
  }
}

impl From<Error> for wasmrs_frames::PayloadError {
  fn from(val: Error) -> Self {
    wasmrs_frames::PayloadError::new(0, val.to_string())
  }
}
