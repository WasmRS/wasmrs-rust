//! Library-specific error types and utility functions

/// Error type for wasmRS errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Initialization Failed.
  #[error("Initialization failed: {0}")]
  InitFailed(String),

  /// Creating a new context failed.
  #[error("Could not create new context: {0}")]
  NewContext(String),

  /// Sending a frame failed.
  #[error("Could not send frame: {0}")]
  SendFailed(String),

  /// Guest send response to a stream that doesn't exist.
  #[error(transparent)]
  RSocket(#[from] wasmrs::Error),

  /// Guest send response to a stream that doesn't exist.
  #[error(transparent)]
  PayloadError(#[from] wasmrs_frames::PayloadError),

  /// Querying Operation List failed.
  #[error("Failed to query or decode operation list")]
  OpList(String),

  /// Could not find specified operation.
  #[error("Could not find operation {0}::{1}")]
  OpMissing(String, String),
}
