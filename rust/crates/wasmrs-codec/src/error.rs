#[derive(Debug)]
pub enum Error {
  MsgPackDecode(wasm_msgpack::decode::Error),
  MsgPackEncode(wasm_msgpack::encode::Error),
}

// #[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::MsgPackDecode(e) => f.write_str(e.to_string().as_str()),
      Error::MsgPackEncode(e) => f.write_str(e.to_string().as_str()),
    }
  }
}
