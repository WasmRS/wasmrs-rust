#[cfg(feature = "serde")]
pub mod serde;

use crate::marker::Marker;

use byteorder::{BigEndian, ByteOrder};
#[allow(unused_imports)]
use core::convert::TryFrom;
use core::{convert::From, ops::Deref};
use num_traits::cast::FromPrimitive;

/// Error type indicating why serialization failed
#[derive(Debug)]
pub enum Error {
    /// End of buffer was reached, before object could be serialized.
    EndOfBuffer,
    /// Value was out of bounds.
    OutOfBounds,
    /// Happens if the data type can not be serialized. For example if a sequence is not sized.
    InvalidType,
}

impl ::core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::OutOfBounds => f.write_str("Out of bounds"),
            Error::InvalidType => f.write_str("Invalid type"),
            Error::EndOfBuffer => f.write_str("End of buffer"),
        }
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {}

pub trait SerializeIntoSlice {
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error>;
}

pub fn serialize_u8(value: u8, buf: &mut [u8]) -> Result<usize, Error> {
    if value < 0x80 {
        if buf.is_empty() {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = value;
        Ok(1)
    } else {
        if buf.len() < 2 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U8.to_u8();
        buf[1] = value;
        Ok(2)
    }
}
pub fn serialize_u16(value: u16, buf: &mut [u8]) -> Result<usize, Error> {
    if let Ok(value) = u8::try_from(value) {
        serialize_u8(value, buf)
    } else {
        if buf.len() < 3 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U16.to_u8();
        BigEndian::write_u16(&mut buf[1..], value);
        Ok(3)
    }
}
pub fn serialize_u32(value: u32, buf: &mut [u8]) -> Result<usize, Error> {
    if let Ok(value) = u16::try_from(value) {
        serialize_u16(value, buf)
    } else {
        if buf.len() < 5 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U32.to_u8();
        BigEndian::write_u32(&mut buf[1..], value);
        Ok(5)
    }
}
#[cfg(feature = "u64")]
pub fn serialize_u64(value: u64, buf: &mut [u8]) -> Result<usize, Error> {
    if let Ok(value) = u32::try_from(value) {
        serialize_u32(value, buf)
    } else {
        if buf.len() < 9 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::U64.to_u8();
        BigEndian::write_u64(&mut buf[1..], value);
        Ok(9)
    }
}
#[allow(clippy::single_match_else)]
pub fn serialize_i8(value: i8, buf: &mut [u8]) -> Result<usize, Error> {
    match value {
        -32..=0x7f => {
            if buf.is_empty() {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = value as u8;
            Ok(1)
        }
        _ => {
            if buf.len() < 2 {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = Marker::I8.to_u8();
            buf[1] = value as u8;
            Ok(2)
        }
    }
}
pub fn serialize_i16(value: i16, buf: &mut [u8]) -> Result<usize, Error> {
    // if value <= i8::max_value() as i16 && value >= i8::min_value() as i16 {
    if let Some(value) = u16::from_i16(value) {
        serialize_u16(value, buf)
    } else if let Some(value) = i8::from_i16(value) {
        serialize_i8(value, buf)
    } else {
        if buf.len() < 3 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::I16.to_u8();
        BigEndian::write_i16(&mut buf[1..], value);
        Ok(3)
    }
}
pub fn serialize_i32(value: i32, buf: &mut [u8]) -> Result<usize, Error> {
    // if value <= i16::max_value() as i32 && value >= i16::min_value() as i32 {
    if let Some(value) = u32::from_i32(value) {
        serialize_u32(value, buf)
    } else if let Some(value) = i16::from_i32(value) {
        serialize_i16(value, buf)
    } else {
        if buf.len() < 5 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::I32.to_u8();
        BigEndian::write_i32(&mut buf[1..], value);
        Ok(5)
    }
}
#[cfg(feature = "i64")]
pub fn serialize_i64(value: i64, buf: &mut [u8]) -> Result<usize, Error> {
    #[cfg(feature = "u64")]
    if let Some(value) = u64::from_i64(value) {
        return serialize_u64(value, buf);
    }
    if let Some(value) = i32::from_i64(value) {
        serialize_i32(value, buf)
    } else {
        if buf.len() < 9 {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::I64.to_u8();
        BigEndian::write_i64(&mut buf[1..], value);
        Ok(9)
    }
}
pub fn serialize_f32(value: f32, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 5 {
        return Err(Error::EndOfBuffer);
    }
    buf[0] = Marker::F32.to_u8();
    BigEndian::write_f32(&mut buf[1..], value);
    Ok(5)
}
pub fn serialize_f64(value: f64, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 9 {
        return Err(Error::EndOfBuffer);
    }
    buf[0] = Marker::F64.to_u8();
    BigEndian::write_f64(&mut buf[1..], value);
    Ok(9)
}

impl SerializeIntoSlice for u8 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_u8(*self, buf) }
}
impl SerializeIntoSlice for u16 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_u16(*self, buf) }
}
impl SerializeIntoSlice for u32 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_u32(*self, buf) }
}
#[cfg(feature = "u64")]
impl SerializeIntoSlice for u64 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_u64(*self, buf) }
}
impl SerializeIntoSlice for i8 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_i8(*self, buf) }
}
impl SerializeIntoSlice for i16 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_i16(*self, buf) }
}
impl SerializeIntoSlice for i32 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_i32(*self, buf) }
}
#[cfg(feature = "i64")]
impl SerializeIntoSlice for i64 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_i64(*self, buf) }
}

impl SerializeIntoSlice for f32 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_f32(*self, buf) }
}
impl SerializeIntoSlice for f64 {
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_f64(*self, buf) }
}

impl SerializeIntoSlice for bool {
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = if *self { Marker::True.to_u8() } else { Marker::False.to_u8() };
        Ok(1)
    }
}

impl<T> SerializeIntoSlice for Option<T>
where T: SerializeIntoSlice
{
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if let Some(value) = self {
            SerializeIntoSlice::write_into_slice(value, buf)
        } else {
            if buf.is_empty() {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = Marker::Null.to_u8();
            Ok(1)
        }
    }
}
impl SerializeIntoSlice for () {
    #[inline(always)]
    fn write_into_slice(&self, _buf: &mut [u8]) -> Result<usize, Error> { Ok(0) }
}

#[repr(transparent)]
#[cfg_attr(feature = "std", derive(core::fmt::Debug))]
pub struct Binary<'a>(&'a [u8]);
impl<'a> Binary<'a> {
    pub const fn new(slice: &'a [u8]) -> Self { Binary(slice) }
}
impl<'a> Deref for Binary<'a> {
    type Target = &'a [u8];
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<'a> From<&'a [u8]> for Binary<'a> {
    fn from(slice: &'a [u8]) -> Self { Binary(slice) }
}
impl<'a> PartialEq for Binary<'a> {
    fn eq(&self, other: &Binary<'a>) -> bool { self.0 == other.0 }
}

#[cfg(feature = "serde")]
impl<'a> ::serde::Serialize for Binary<'a> {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_bytes(self.0) }
}
#[cfg(feature = "serde")]
struct BinaryVisitor;

#[cfg(feature = "serde")]
impl<'de> ::serde::de::Visitor<'de> for BinaryVisitor {
    type Value = Binary<'de>;
    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result { formatter.write_str("a byte array") }
    fn visit_borrowed_bytes<E: ::serde::de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> { Ok(Binary::new(v)) }
}
#[cfg(feature = "serde")]
impl<'a> ::serde::Deserialize<'a> for Binary<'a> {
    fn deserialize<D: ::serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_bytes(BinaryVisitor)
    }
}

/// # Panics
///
/// Will panic under the following conditions:
///  - feature 'bin32' active: `n >= 2^32`
///  - feature 'bin16' active: `n >= 2^16`
///  - else: `n >= 2^8`
impl<'a> SerializeIntoSlice for Binary<'a> {
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = self.len();
        if let Ok(n8) = u8::try_from(n) {
            if buf.len() < 2 + n {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = Marker::Bin8.to_u8();
            buf[1] = n8;
            buf[2..(2 + n)].clone_from_slice(self);
            return Ok(2 + n);
        }
        #[cfg(feature = "bin16")]
        if let Ok(n16) = u16::try_from(n) {
            if buf.len() < 3 + n {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = Marker::Bin16.to_u8();
            BigEndian::write_u16(&mut buf[1..], n16);
            buf[3..(3 + n)].clone_from_slice(self);
            return Ok(3 + n);
        }
        #[cfg(feature = "bin32")]
        if let Ok(n32) = u32::try_from(n) {
            if buf.len() < 5 + n {
                return Err(Error::EndOfBuffer);
            }
            buf[0] = Marker::Bin32.to_u8();
            BigEndian::write_u32(&mut buf[1..], n32);
            buf[5..(5 + n)].clone_from_slice(self);
            return Ok(5 + n);
        }
        unimplemented!()
    }
}

impl<K, V> SerializeIntoSlice for &(K, V)
where
    K: SerializeIntoSlice,
    V: SerializeIntoSlice,
{
    #[inline(always)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let index = serialize_map_kay_value(&self.0, &self.1, buf)?;
        Ok(index)
    }
}

/// # Panics
///
/// Will panic under the following conditions:
///  - feature 'str32' active: `n >= 2^32`
///  - feature 'str16' active: `n >= 2^16`
///  - else: `n >= 2^8`
impl SerializeIntoSlice for &str {
    #[allow(clippy::cast_possible_truncation)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = self.len();
        match n {
            // FIXSTR_SIZE
            0..=0x1f => {
                let header_len = 1;
                if buf.len() < header_len + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::FixStr(n as u8).to_u8();
                buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                Ok(header_len + n)
            }
            0x20..=0xff => {
                let header_len = 2;
                if buf.len() < header_len + n {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = Marker::Str8.to_u8();
                buf[1] = n as u8;
                buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                Ok(header_len + n)
            }
            _ => {
                #[cfg(feature = "str16")]
                if let Ok(n16) = u16::try_from(n) {
                    let header_len = 3;
                    if buf.len() < header_len + n {
                        return Err(Error::EndOfBuffer);
                    }
                    buf[0] = Marker::Str16.to_u8();
                    BigEndian::write_u16(&mut buf[1..], n16);
                    buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                    return Ok(header_len + n);
                }
                #[cfg(feature = "str32")]
                if let Ok(n32) = u32::try_from(n) {
                    let header_len = 5;
                    if buf.len() < header_len + n {
                        return Err(Error::EndOfBuffer);
                    }
                    buf[0] = Marker::Str32.to_u8();
                    BigEndian::write_u32(&mut buf[1..], n32);
                    buf[header_len..(header_len + n)].clone_from_slice(self.as_bytes());
                    return Ok(header_len + n);
                }
                unimplemented!()
            }
        }
    }
}

impl<K, V> SerializeIntoSlice for &[(K, V)]
where
    K: SerializeIntoSlice,
    V: SerializeIntoSlice,
{
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        // serialize_sequence(self, SequenceType::Map, buf)
        let mut index = serialize_map_start(self.len(), buf)?;
        for kv in self.iter() {
            index += kv.write_into_slice(&mut buf[index..])?;
        }
        Ok(index)
    }
}

impl<T> SerializeIntoSlice for &[T]
where T: SerializeIntoSlice
{
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        // serialize_sequence(self, SequenceType::Array, buf)
        let mut index = serialize_array_start(self.len(), buf)?;
        for i in self.iter() {
            index += SerializeIntoSlice::write_into_slice(i, &mut buf[index..])?;
        }
        Ok(index)
    }
}

#[derive(Copy, Clone)]
pub enum SequenceType {
    Array,
    Map,
}
impl SequenceType {
    pub fn serialize_start(self, n: usize, buf: &mut [u8]) -> Result<usize, Error> {
        match self {
            SequenceType::Array => serialize_array_start(n, buf),
            SequenceType::Map => serialize_map_start(n, buf),
        }
    }
}

pub fn serialize_sequence<T: SerializeIntoSlice>(seq: &[T], typ: SequenceType, buf: &mut [u8]) -> Result<usize, Error> {
    let mut index = typ.serialize_start(seq.len(), buf)?;
    for i in seq.iter() {
        index += SerializeIntoSlice::write_into_slice(i, &mut buf[index..])?;
    }
    Ok(index)
}

/// # Panics
///
/// Will panic under the following conditions:
///  - feature 'array32' active: `n >= 2^32`
///  - feature 'array16' active: `n >= 2^16`
///  - else: `n >= 16`
#[allow(clippy::cast_possible_truncation)]
pub fn serialize_array_start(n: usize, buf: &mut [u8]) -> Result<usize, Error> {
    if n <= crate::marker::FIXARRAY_SIZE as usize {
        if buf.len() < 1 + n {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::FixArray(n as u8).to_u8();
        Ok(1)
    } else {
        #[cfg(feature = "array16")]
        if let Ok(n) = u16::try_from(n) {
            buf[0] = Marker::Array16.to_u8();
            BigEndian::write_u16(&mut buf[1..], n);
            return Ok(3);
        }
        #[cfg(feature = "array32")]
        if let Ok(n) = u32::try_from(n) {
            buf[0] = Marker::Array32.to_u8();
            BigEndian::write_u32(&mut buf[1..], n);
            return Ok(5);
        }
        unimplemented!()
    }
}

/// # Panics
///
/// Will panic under the following conditions:
///  - feature 'map32' active: `n >= 2^32`
///  - feature 'map16' active: `n >= 2^16`
///  - else: `n >= 16`
#[allow(clippy::cast_possible_truncation)]
pub fn serialize_map_start(n: usize, buf: &mut [u8]) -> Result<usize, Error> {
    if n <= crate::marker::FIXMAP_SIZE as usize {
        if buf.len() < 1 + n {
            return Err(Error::EndOfBuffer);
        }
        buf[0] = Marker::FixMap(n as u8).to_u8();
        Ok(1)
    } else {
        #[cfg(feature = "map16")]
        if let Ok(n) = u16::try_from(n) {
            buf[0] = Marker::Map16.to_u8();
            BigEndian::write_u16(&mut buf[1..], n);
            return Ok(3);
        }
        #[cfg(feature = "map32")]
        if let Ok(n) = u32::try_from(n) {
            buf[0] = Marker::Map32.to_u8();
            BigEndian::write_u32(&mut buf[1..], n);
            return Ok(5);
        }
        unimplemented!()
    }
}
pub fn serialize_map_kay_value<K: SerializeIntoSlice, V: SerializeIntoSlice>(key: &K, value: &V, buf: &mut [u8]) -> Result<usize, Error> {
    let mut index = 0;
    index += SerializeIntoSlice::write_into_slice(key, &mut buf[index..])?;
    index += SerializeIntoSlice::write_into_slice(value, &mut buf[index..])?;
    Ok(index)
}
