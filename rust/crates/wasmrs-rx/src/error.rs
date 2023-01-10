//! Library-specific error types and utility functions

/// Error type for wasmRS-rx errors.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub enum Error {
  /// Receive on a channel failed.
  RecvFailed(u8),
  /// The receiver in a [Flux] has already been removed.
  ReceiverAlreadyGone,
  /// A Runtime-related error.
  Runtime(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::RecvFailed(_) => f.write_str("Receive failed"),
      Error::ReceiverAlreadyGone => f.write_str("Received already taken"),
      Error::Runtime(msg) => f.write_str(msg),
    }
  }
}

impl From<wasmrs_runtime::Error> for Error {
  fn from(e: wasmrs_runtime::Error) -> Self {
    Self::Runtime(e.to_string())
  }
}
