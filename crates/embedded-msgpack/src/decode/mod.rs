#[cfg(feature = "serde")]
mod serde;

use crate::marker::Marker;

use byteorder::{BigEndian, ByteOrder};
use num_traits::cast::FromPrimitive;
use zerocopy::ByteSlice;

/// Error type indicating why deserialization failed
#[derive(Debug)]
pub enum Error {
    /// End of buffer was reached, before object was deserialized.
    EndOfBuffer,
    /// Value was out of bounds. This can happen if for example a `u32` value is deserialized into `u16`.
    ///
    /// # Examples
    ///
    /// `[0xcd, 0x12, 0x34]` deserializes to `0x1234`. If you try to serialize into `u8`, this error will occur.
    OutOfBounds,
    /// Happens if the data type does not match the expected type.
    InvalidType,
    CustomError,
    #[cfg(feature = "custom-error-messages")]
    CustomErrorWithMessage(heapless::String<64>),
}

#[cfg(feature = "serde")]
// #[inline(never)]
pub fn from_slice<'a, T: ::serde::de::Deserialize<'a>>(buf: &'a [u8]) -> Result<T, Error> {
    let mut de = serde::Deserializer::new(buf);
    let value = ::serde::de::Deserialize::deserialize(&mut de)?;

    Ok(value)
}

pub trait DeserializeFromSlice {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error>;
}

impl DeserializeFromSlice for Option<u8> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_u8(buf)?;
        *self = Some(v);
        Ok(n)
    }
}
impl DeserializeFromSlice for Option<u16> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_u16(buf)?;
        *self = Some(v);
        Ok(n)
    }
}
impl DeserializeFromSlice for Option<u32> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_u32(buf)?;
        *self = Some(v);
        Ok(n)
    }
}
impl DeserializeFromSlice for Option<u64> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_u64(buf)?;
        *self = Some(v);
        Ok(n)
    }
}
impl DeserializeFromSlice for Option<i8> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_i8(buf)?;
        *self = Some(v);
        Ok(n)
    }
}
impl DeserializeFromSlice for Option<i16> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_i16(buf)?;
        *self = Some(v);
        Ok(n)
    }
}
impl DeserializeFromSlice for Option<i32> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_i32(buf)?;
        *self = Some(v);
        Ok(n)
    }
}
impl DeserializeFromSlice for Option<i64> {
    fn from_slice(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let (v, n) = read_i64(buf)?;
        *self = Some(v);
        Ok(n)
    }
}

pub fn read_raw_u8(buf: &[u8]) -> Result<(u8, &[u8]), Error> { buf.split_first().map(|(&x, rest)| (x, rest)).ok_or(Error::EndOfBuffer) }
pub fn read_raw_u16<B: ByteSlice>(buf: B) -> Result<(u16, B), Error> {
    // pub fn read_raw_u16(buf: &[u8]) -> Result<(u16, &[u8]), Error> {
    if buf.len() < 2 {
        return Err(Error::EndOfBuffer);
    }
    let (v, rest) = buf.split_at(2);
    Ok((BigEndian::read_u16(&*v), rest))
}
pub fn read_raw_u32(buf: &[u8]) -> Result<(u32, &[u8]), Error> {
    if buf.len() < 4 {
        return Err(Error::EndOfBuffer);
    }
    let (v, rest) = buf.split_at(4);
    Ok((BigEndian::read_u32(v), rest))
}
#[cfg(feature = "u64")]
pub fn read_raw_u64(buf: &[u8]) -> Result<(u64, &[u8]), Error> {
    if buf.len() < 8 {
        return Err(Error::EndOfBuffer);
    }
    let (v, rest) = buf.split_at(8);
    Ok((BigEndian::read_u64(v), rest))
}
#[allow(dead_code)]
fn read_raw_ux(buf: &[u8], num_bytes: u8) -> Result<(usize, &[u8]), Error> {
    Ok(match num_bytes {
        1 => {
            let (x, rest) = read_raw_u8(buf)?;
            (x as usize, rest)
        }
        2 => {
            let (x, rest) = read_raw_u16(buf)?;
            (x as usize, rest)
        }
        4 => {
            let (x, rest) = read_raw_u32(buf)?;
            (x as usize, rest)
        }
        _ => unreachable!(),
    })
}

pub fn read_int<B: ByteSlice, T: FromPrimitive>(buf: B) -> Result<(T, usize), Error> {
    match read_u64(buf) {
        Ok((v, len)) => T::from_u64(v).map_or(Err(Error::OutOfBounds), |v| Ok((v, len))),
        Err(kind) => Err(kind),
    }
}
#[cfg(feature = "i64")]
pub fn read_sint<B: ByteSlice, T: FromPrimitive>(buf: B) -> Result<(T, usize), Error> {
    match read_i64(buf) {
        Ok((v, len)) => T::from_i64(v).map_or(Err(Error::OutOfBounds), |v| Ok((v, len))),
        Err(kind) => Err(kind),
    }
}
#[cfg(not(feature = "i64"))]
pub fn read_sint<B: ByteSlice, T: FromPrimitive>(buf: B) -> Result<(T, usize), Error> {
    match read_i32(buf) {
        Ok((v, len)) => {
            if let Some(v) = T::from_i32(v) {
                Ok((v, len))
            } else {
                Err(Error::OutOfBounds)
            }
        }
        Err(kind) => Err(kind),
    }
}

#[inline(always)]
pub fn read_u8<B: ByteSlice>(buf: B) -> Result<(u8, usize), Error> { read_int(buf) }
#[inline(always)]
pub fn read_u16<B: ByteSlice>(buf: B) -> Result<(u16, usize), Error> { read_int(buf) }
#[inline(always)]
pub fn read_u32<B: ByteSlice>(buf: B) -> Result<(u32, usize), Error> { read_int(buf) }

#[inline(always)]
pub fn read_i8<B: ByteSlice>(buf: B) -> Result<(i8, usize), Error> { read_sint(buf) }
#[inline(always)]
pub fn read_i16<B: ByteSlice>(buf: B) -> Result<(i16, usize), Error> { read_sint(buf) }
#[inline(always)]
pub fn read_i32<B: ByteSlice>(buf: B) -> Result<(i32, usize), Error> { read_sint(buf) }

pub fn read_bool<B: ByteSlice>(buf: B) -> Result<(bool, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    match Marker::from(buf[0]) {
        Marker::True => Ok((true, 1)),
        Marker::False => Ok((false, 1)),
        _ => Err(Error::InvalidType),
    }
}

pub fn read_u64<B: ByteSlice>(buf: B) -> Result<(u64, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        // Nur u64 muss hier gesondert behandelt werden, weil es der einzige Typ ist, der potentiell nicht in i64 passt
        Marker::U64 => {
            if buf.len() >= 9 {
                Ok((BigEndian::read_u64(&buf[1..9]) as u64, 9))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => match read_i64(buf) {
            Ok((i, l)) => u64::from_i64(i).map_or(Err(Error::OutOfBounds), |u| Ok((u, l))),
            Err(kind) => Err(kind),
        },
    }
}

pub fn read_i64<B: ByteSlice>(buf: B) -> Result<(i64, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::FixPos(val) => Ok((i64::from(val), 1)),
        Marker::FixNeg(val) => Ok((i64::from(val), 1)),

        Marker::U8 => {
            if buf.len() >= 2 {
                Ok((i64::from(buf[1]), 2))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::U16 => {
            if buf.len() >= 3 {
                Ok((i64::from(BigEndian::read_u16(&buf[1..3])), 3))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::U32 => {
            if buf.len() >= 5 {
                Ok((i64::from(BigEndian::read_u32(&buf[1..5])), 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::U64 => {
            if buf.len() >= 9 {
                let u = BigEndian::read_u64(&buf[1..9]);
                i64::from_u64(u).map_or(Err(Error::OutOfBounds), |i| Ok((i, 9)))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        #[allow(clippy::cast_possible_wrap)]
        Marker::I8 => {
            if buf.len() >= 2 {
                Ok((i64::from(buf[1] as i8), 2))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::I16 => {
            if buf.len() >= 3 {
                Ok((i64::from(BigEndian::read_i16(&buf[1..3])), 3))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::I32 => {
            if buf.len() >= 5 {
                Ok((i64::from(BigEndian::read_i32(&buf[1..5])), 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::I64 => {
            if buf.len() >= 9 {
                Ok((BigEndian::read_i64(&buf[1..9]), 9))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::EndOfBuffer),
    }
}

pub fn read_f32<B: ByteSlice>(buf: B) -> Result<(f32, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::F32 => {
            if buf.len() >= 5 {
                Ok((BigEndian::read_f32(&buf[1..5]), 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::EndOfBuffer),
    }
}
pub fn read_f64<B: ByteSlice>(buf: B) -> Result<(f64, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::F32 => {
            if buf.len() >= 5 {
                let v = BigEndian::read_f32(&buf[1..5]);
                Ok((f64::from(v), 5))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::F64 => {
            if buf.len() >= 9 {
                Ok((BigEndian::read_f64(&buf[1..9]), 9))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::EndOfBuffer),
    }
}

pub fn read_bin<B: ByteSlice>(buf: B) -> Result<(B, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    match marker {
        Marker::FixStr(len) => {
            let header_len = 1;
            let len = len as usize;
            if buf.len() >= header_len + len {
                let (_head, rest) = buf.split_at(header_len);
                let (bin, _rest) = rest.split_at(len);
                Ok((bin, header_len + len))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::Bin8 | Marker::Str8 => {
            let header_len = 2;
            if let Some(&len) = buf.get(1) {
                let len = len as usize;
                if buf.len() >= header_len + len {
                    let (_head, rest) = buf.split_at(header_len);
                    let (bin, _rest) = rest.split_at(len);
                    Ok((bin, header_len + len))
                } else {
                    Err(Error::EndOfBuffer)
                }
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        Marker::Bin16 | Marker::Str16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let (_, buf) = buf.split_at(1);
            let (len, buf) = read_raw_u16(buf)?; //BigEndian::read_u16(&buf[1..header_len]) as usize;
            let len = len as usize;
            if buf.len() >= len {
                let (bin, _rest) = buf.split_at(len);
                Ok((bin, header_len + len))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        #[cfg(feature = "bin32")]
        Marker::Bin32 | Marker::Str32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                let (_head, rest) = buf.split_at(header_len);
                let (bin, _rest) = rest.split_at(len);
                Ok((bin, header_len + len))
            } else {
                Err(Error::EndOfBuffer)
            }
        }
        _ => Err(Error::InvalidType),
    }
}

pub fn read_str(buf: &[u8]) -> Result<(&str, usize), Error> {
    if buf.is_empty() {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    let (header_len, len) = match marker {
        Marker::FixStr(len) => {
            let header_len = 1;
            let len = len as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        Marker::Str8 => {
            let header_len = 2;
            if let Some(&len) = buf.get(1) {
                let len = len as usize;
                if buf.len() >= header_len + len {
                    (header_len, len)
                } else {
                    return Err(Error::EndOfBuffer);
                }
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        Marker::Str16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u16(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "str32")]
        Marker::Str32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        _ => return Err(Error::InvalidType),
    };
    let buf = &buf[header_len..header_len + len];
    let s = if buf.is_ascii() {
        // This is safe because all ASCII characters are valid UTF-8 characters
        unsafe { core::str::from_utf8_unchecked(buf) }
    } else {
        return Err(Error::InvalidType);
    };
    Ok((s, header_len + len))
}

pub fn read_array_len<B: ByteSlice>(buf: B) -> Result<(usize, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    // let (&marker, buf) = buf.split_first().ok_or(Error::EndOfBuffer)?;
    let marker = Marker::from(buf[0]);
    let (header_len, len) = match marker {
        Marker::FixArray(len) => {
            let header_len = 1;
            let len = len as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "array16")]
        Marker::Array16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u16(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        #[cfg(feature = "array32")]
        Marker::Array32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            if buf.len() >= header_len + len {
                (header_len, len)
            } else {
                return Err(Error::EndOfBuffer);
            }
        }
        _ => return Err(Error::InvalidType),
    };
    Ok((len, header_len))
}

pub fn read_map_len<B: ByteSlice>(buf: B) -> Result<(usize, usize), Error> {
    if buf.len() == 0 {
        return Err(Error::EndOfBuffer);
    }

    let marker = Marker::from(buf[0]);
    let (len, header_len) = match marker {
        Marker::FixMap(len) => {
            let header_len = 1;
            let len = len as usize;
            (len, header_len)
        }
        #[cfg(feature = "map16")]
        Marker::Map16 => {
            let header_len = 3;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u16(&buf[1..header_len]) as usize;
            (len, header_len)
        }
        #[cfg(feature = "map32")]
        Marker::Map32 => {
            let header_len = 5;
            if buf.len() < header_len {
                return Err(Error::EndOfBuffer);
            }
            let len = BigEndian::read_u32(&buf[1..header_len]) as usize;
            (len, header_len)
        }
        _ => return Err(Error::InvalidType),
    };
    if buf.len() >= header_len + len {
        Ok((len, header_len))
    } else {
        Err(Error::EndOfBuffer)
    }
}

pub fn skip_any<B: ByteSlice>(buf: B) -> Result<((), usize), Error> {
    if buf.is_empty() {
        return Ok(((), 0));
    }
    let marker = Marker::from_u8(buf[0]);
    let n = match marker {
        Marker::FixPos(_) => 1,
        Marker::U8 => 2,
        Marker::U16 => 3,
        Marker::U32 => 5,
        Marker::U64 => 9,
        Marker::FixNeg(_) => 1,
        Marker::I8 => 2,
        Marker::I16 => 3,
        Marker::I32 => 5,
        Marker::I64 => 9,

        Marker::F32 => 5,
        Marker::F64 => 9,

        Marker::Null | Marker::True | Marker::False | Marker::Reserved => 1,

        Marker::FixStr(n) => n as usize + 1,
        Marker::Str8 | Marker::Bin8 => {
            if buf.len() < 2 {
                return Err(Error::EndOfBuffer);
            }
            2 + buf[1] as usize
        }
        Marker::Str16 | Marker::Bin16 => {
            if buf.len() < 3 {
                return Err(Error::EndOfBuffer);
            }
            3 + BigEndian::read_u16(&buf[1..3]) as usize
        }
        Marker::Str32 | Marker::Bin32 => {
            if buf.len() < 5 {
                return Err(Error::EndOfBuffer);
            }
            5 + BigEndian::read_u32(&buf[1..5]) as usize
        }

        Marker::FixArray(_) | Marker::Array16 | Marker::Array32 => {
            let (len, n) = read_array_len(&buf[..])?;
            let mut n = n;
            for _ in 0..len {
                //TODO: May overflow stack on embedded systems. Maybe add some kind of safeguard to limit recursion depth
                n += skip_any(&buf[n..])?.1;
            }
            n
        }
        Marker::FixMap(_) | Marker::Map16 | Marker::Map32 => {
            let (len, n) = read_map_len(&buf[..])?;
            let mut n = n;
            for _ in 0..len * 2 {
                //TODO: May overflow stack on embedded systems. Maybe add some kind of safeguard to limit recursion depth
                n += skip_any(&buf[n..])?.1;
            }
            n
        }
        Marker::FixExt1 => 3,
        Marker::FixExt2 => 4,
        Marker::FixExt4 => 6,
        Marker::FixExt8 => 10,
        Marker::FixExt16 => 18,
        Marker::Ext8 => {
            if buf.len() < 2 {
                return Err(Error::EndOfBuffer);
            }
            3 + buf[1] as usize
        }
        Marker::Ext16 => {
            if buf.len() < 3 {
                return Err(Error::EndOfBuffer);
            }
            4 + BigEndian::read_u16(&buf[1..3]) as usize
        }
        Marker::Ext32 => {
            if buf.len() < 5 {
                return Err(Error::EndOfBuffer);
            }
            6 + BigEndian::read_u32(&buf[1..5]) as usize
        }
    };
    if buf.len() < n {
        return Err(Error::EndOfBuffer);
    }
    Ok(((), n))
}
