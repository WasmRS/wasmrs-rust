//! Library-specific error types and utility functions

/// Error type for wasmRS-rx errors.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub enum Error {
  /// Receive on a channel failed.
  RecvFailed(u8),
  /// The sender in a [FluxChannel] has already been removed or dropped.
  SenderClosed,
  /// The receiver in a [FluxChannel] has already been removed.
  ReceiverAlreadyGone,
  /// A Runtime-related error.
  Runtime(String),
  /// Error decoding a payload.
  Decode(String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::RecvFailed(_) => f.write_str("Receive failed"),
      Error::ReceiverAlreadyGone => f.write_str("Received already taken"),
      Error::Decode(e) => {
        let mut message = "Decode error: ".to_owned();
        message.push_str(e);
        f.write_str(e)
      }
      Error::Runtime(msg) => f.write_str(msg),
      Error::SenderClosed => f.write_str("Sender closed"),
    }
  }
}

impl From<wasmrs_runtime::Error> for Error {
  fn from(e: wasmrs_runtime::Error) -> Self {
    Self::Runtime(e.to_string())
  }
}
