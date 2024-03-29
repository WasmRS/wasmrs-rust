use serde::{Deserialize, Serialize};

use crate::error::Error;
use core::result::Result;
extern crate alloc;
use alloc::vec::Vec;

#[doc(hidden)]
pub fn mp_serialize<T>(item: &T) -> Result<Vec<u8>, rmp_serde::encode::Error>
where
  T: ?Sized + Serialize,
{
  let mut buf = Vec::new();
  let mut serializer = rmp_serde::encode::Serializer::new(&mut buf)
    .with_human_readable()
    .with_struct_map();
  item.serialize(&mut serializer)?;
  Ok(buf)
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
pub fn mp_deserialize<'de, T: Deserialize<'de>>(buf: &'de [u8]) -> Result<T, rmp_serde::decode::Error> {
  rmp_serde::decode::from_slice(buf)
}

/// The standard function for de-serializing codec structs from a format suitable.
/// for message exchange between actor and host. Use of any other function to.
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(buf: &'de [u8]) -> Result<T, crate::error::Error> {
  mp_deserialize(buf).map_err(Error::MsgPackDecode)
}

#[cfg(test)]
mod test {
  use super::*;
  use bytes::Bytes;

  #[test]
  fn test_bytes() {
    let bytes = b"\xc4\xf2PK\x03\x04\x14\0\0\0\x08\x000t\nA~\xe7\xffi$\0\0\0$\0\0\0\x06\0\0\0README\x0b\xc9\xc8,V(\xceM\xcc\xc9QH\xcb\xccIU\0\xf22\xf3\x14\xa2<\x03\xccL\x14\xd2\xf2\x8br\x13K\xf4\xb8\0PK\x01\x02-\x03-\0\0\0\x08\x000t\nA~\xe7\xffi\xff\xff\xff\xff\xff\xff\xff\xff\x06\0\x14\0\0\0\0\0\0\0\0\0\xa4\x81\0\0\0\0README\x01\0\x10\0$\0\0\0\0\0\0\0$\0\0\0\0\0\0\0PK\x06\x06,\0\0\0\0\0\0\0-\0-\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0H\0\0\0\0\0\0\0H\0\0\0\0\0\0\0PK\x06\x07\0\0\0\0\x90\0\0\0\0\0\0\0\x01\0\0\0PK\x05\x06\0\0\0\0\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\0\0";
    let v: Bytes = deserialize(bytes).unwrap();
    assert_eq!(v, Bytes::from(&bytes[2..]));
  }

  #[test]
  fn test_map() {
    let bytes = b"\x81\xa6source\xa9zip64.zip";
    let actual: serde_json::Value = deserialize(bytes).unwrap();
    let expected = serde_json::json!({
      "source": "zip64.zip"
    });
    assert_eq!(expected, actual);
  }
}
