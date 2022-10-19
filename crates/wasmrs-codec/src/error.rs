#[derive(Debug)]
pub enum Error {
    MsgPackDecode,
    MsgPackEncode,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MsgPackDecode => f.write_str("Could not decode MessagePack bytes"),
            Error::MsgPackEncode => f.write_str("Could not encode data to MessagePack"),
        }
    }
}
