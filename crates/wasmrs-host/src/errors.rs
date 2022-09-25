//! Library-specific error types and utility functions

/// Error type for wasmRS errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Context creation failed.
    #[error("Context creation failed: {0}")]
    Context(String),

    /// Initialization Failed.
    #[error("Initialization failed: {0}")]
    InitFailed(String),

    /// Creating a new context failed.
    #[error("Could not create new context: {0}")]
    NewContext(String),

    /// Error while sending a frame to the guest.
    #[error("Guest send failure: {0}")]
    SendFailure(String),

    /// Guest send response to a stream that doesn't exist.
    #[error("Guest send response to a stream ({0}) that doesn't exist.")]
    StreamNotFound(u32),

    /// Error sending to a handler stream.
    #[error("Error sending a result to handler stream.")]
    StreamSend,

    /// Guest send response to a stream that doesn't exist.
    #[error(transparent)]
    RSocket(#[from] wasmrs_rsocket::error::Error),
}
