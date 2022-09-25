//! Library-specific error types and utility functions

use crate::ErrorCode;

/// Error type for wasmRS RSocket errors.
#[derive(Debug, Clone)]
pub enum Error {
    RSocket(u32),
    RequestResponse(String),
    RequestChannel(String),
    RequestStream(String),
    RequestFnF(String),
    RequestError(u32, String),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RSocket(code) => f.write_str((Into::<u32>::into(*code)).to_string().as_str()),
            _ => f.write_str("Error"),
        }
    }
}
