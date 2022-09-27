#[derive(Debug)]
pub enum Error {
    MsgPackDecode(embedded_msgpack::decode::Error),
    MsgPackEncode(embedded_msgpack::encode::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MsgPackDecode(e) => writeln!(f, "{}", e),
            Error::MsgPackEncode(e) => writeln!(f, "{}", e),
        }
    }
}
