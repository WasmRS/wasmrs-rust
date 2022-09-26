//! Library-specific error types and utility functions

/// Error type for wasmRS RSocket errors.
#[derive(Debug, Clone)]
pub enum Error {
    RSocket(u32),
    RequestResponse(String),
    RequestChannel(String),
    RequestStream(String),
    RequestFnF(String),
    SendFailed(u8),
    RecvFailed(u8),
    ReceiverAlreadyGone,
    PortNotFound(String),
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

#[derive(Debug)]
#[must_use]
pub struct PayloadError {
    pub code: u32,
    pub msg: String,
}

impl PayloadError {
    pub fn new(code: u32, msg: String) -> Self {
        Self { code, msg }
    }
}
impl std::error::Error for PayloadError {}
impl std::fmt::Display for PayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}
