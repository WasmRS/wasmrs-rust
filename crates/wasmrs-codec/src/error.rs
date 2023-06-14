#[derive(Debug)]
pub enum Error {
  MsgPackDecode(rmp_serde::decode::Error),
  MsgPackEncode(rmp_serde::encode::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Error::MsgPackDecode(e) => e.fmt(f),
      Error::MsgPackEncode(e) => e.fmt(f),
    }
  }
}

impl From<Error> for wasmrs_frames::PayloadError {
  fn from(val: Error) -> Self {
    use core::fmt::Write;

    let mut string: heapless::String<256> = heapless::String::new();
    string.write_fmt(format_args!("{:.256}", val)).unwrap();

    wasmrs_frames::PayloadError::new(0, string.as_str(), None)
  }
}
