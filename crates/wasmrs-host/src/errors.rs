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

    /// Error while sending a frame to the guest.
    #[error("Guest send failure: {0}")]
    SendFailure(String),

    /// Guest send response to a stream that doesn't exist.
    #[error("Guest send response to a stream ({0}) that doesn't exist.")]
    StreamNotFound(u32),
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    fn needs_sync_send<T: Send + Sync>() {}

    #[test]
    fn assert_sync_send() {
        needs_sync_send::<super::Error>();
    }
}
