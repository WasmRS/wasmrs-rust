pub use embedded_msgpack::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[doc(hidden)]
pub fn mp_serialize<T>(item: &T) -> std::result::Result<Vec<u8>, embedded_msgpack::encode::Error>
where
  T: ?Sized + Serialize,
{
  let mut buf = [0; 1024 * 100];
  let written = embedded_msgpack::encode::serde::to_array(item, &mut buf)?;
  Ok(buf[0..written].to_vec())
}

/// The standard function for serializing codec structs into a format that can be.
/// used for message exchange between actor and host. Use of any other function to.
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(item: &T) -> Result<Vec<u8>, crate::error::Error>
where
  T: ?Sized + Serialize,
{
  mp_serialize(item).map_err(Error::MsgPackEncode)
}

#[doc(hidden)]
pub fn mp_deserialize<'de, T: Deserialize<'de>>(
  buf: &'de [u8],
) -> std::result::Result<T, embedded_msgpack::decode::Error> {
  embedded_msgpack::decode::from_slice(buf)
}

/// The standard function for de-serializing codec structs from a format suitable.
/// for message exchange between actor and host. Use of any other function to.
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(buf: &'de [u8]) -> Result<T, crate::error::Error> {
  mp_deserialize(buf).map_err(Error::MsgPackDecode)
}
