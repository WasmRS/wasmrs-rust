//! Library-specific error types and utility functions

use crate::ErrorCode;

/// Error type for wasmRS RSocket errors.
#[derive(Debug, Copy, Clone)]
pub enum Error {
    RSocket(ErrorCode),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RSocket(code) => f.write_str((Into::<u32>::into(*code)).to_string().as_str()),
        }
    }
}
