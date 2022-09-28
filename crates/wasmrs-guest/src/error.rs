#[derive(Debug)]
pub enum Error {
    NoHandler,
    HandlerFail(String),
    StringDecode,
    BufferRead,
    Internal(wasmrs::Error),
    PortNotFound(String),
    MetadataNotFound,
    Codec(String),
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

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error")
    }
}
impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::BufferRead
    }
}
