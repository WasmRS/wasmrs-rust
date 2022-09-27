use byteorder::{BigEndian, ByteOrder};
use core::convert::{TryFrom, TryInto};

use crate::{
    encode::{Error, SerializeIntoSlice},
    Ext,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const EXT_TIMESTAMP: i8 = -1;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Timestamp {
    seconds: i64,
    nanoseconds: u32,
}

impl Timestamp {
    pub const fn new(seconds: i64, nanoseconds: u32) -> Result<Timestamp, Error> {
        if nanoseconds >= 1_000_000_000 {
            return Err(Error::OutOfBounds);
        }
        Ok(Timestamp { seconds, nanoseconds })
    }
    pub const fn seconds(&self) -> i64 { self.seconds }
    pub const fn nanoseconds(&self) -> u32 { self.nanoseconds }
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_ext<'a>(&self, buf: &'a mut [u8]) -> Result<Ext<'a>, Error> {
        if self.seconds >> 34 == 0 {
            let x = (u64::from(self.nanoseconds) << 34) | self.seconds as u64;
            if x & 0xffff_ffff_0000_0000_u64 == 0 {
                // timestamp 32
                if buf.len() < 4 {
                    return Err(Error::EndOfBuffer);
                }
                BigEndian::write_u32(buf, x as u32);
                Ok(Ext::new(-1, &buf[0..4]))
            } else {
                // timestamp 64
                if buf.len() < 8 {
                    return Err(Error::EndOfBuffer);
                }
                BigEndian::write_u64(buf, x);
                Ok(Ext::new(-1, &buf[0..8]))
            }
        } else {
            // timestamp 96
            #[cfg(feature = "timestamp96")]
            {
                if buf.len() < 12 {
                    return Err(Error::EndOfBuffer);
                }
                BigEndian::write_u32(buf, self.nanoseconds);
                BigEndian::write_i64(&mut buf[4..], self.seconds);
                return Ok(Ext::new(-1, &buf[0..12]));
                // serialize(0xc7, 12, -1, time.tv_nsec, time.tv_sec)
            }
            #[cfg(not(feature = "timestamp96"))]
            return Err(Error::InvalidType);
        }
    }
}

impl SerializeIntoSlice for Timestamp {
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.seconds >> 34 == 0 {
            let x = (u64::from(self.nanoseconds) << 34) | self.seconds as u64;
            if x & 0xffff_ffff_0000_0000_u64 == 0 {
                // timestamp 32
                if buf.len() < 6 {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = crate::marker::Marker::FixExt4.to_u8();
                buf[1] = -1i8 as u8;
                BigEndian::write_u32(&mut buf[2..], x as u32);
                Ok(6)
            // serialize(0xd6, -1, x as u32)
            } else {
                // timestamp 64
                if buf.len() < 10 {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = crate::marker::Marker::FixExt8.to_u8();
                buf[1] = -1i8 as u8;
                BigEndian::write_u64(&mut buf[2..], x);
                Ok(10)
                // serialize(0xd7, -1, x)
            }
        } else {
            #[cfg(feature = "timestamp96")]
            return {
                // timestamp 96
                if buf.len() < 12 {
                    return Err(Error::EndOfBuffer);
                }
                buf[0] = crate::marker::Marker::Ext8.to_u8();
                buf[1] = 12;
                buf[2] = -1i8 as u8;
                BigEndian::write_u32(&mut buf[3..], self.nanoseconds);
                BigEndian::write_i64(&mut buf[7..], self.seconds);
                Ok(15)
                // serialize(0xc7, 12, -1, self.nanoseconds, self.seconds)}
            };
            #[cfg(not(feature = "timestamp96"))]
            return Err(Error::InvalidType);
        }
    }
}

impl<'a> TryFrom<Ext<'a>> for Timestamp {
    type Error = Error;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(ext: Ext<'a>) -> Result<Self, Self::Error> {
        if ext.typ == EXT_TIMESTAMP {
            match ext.data.len() {
                4 => {
                    // timestamp 32 stores the number of seconds that have elapsed since 1970-01-01 00:00:00 UTC
                    // in an 32-bit unsigned integer:
                    // +--------+--------+--------+--------+--------+--------+
                    // |  0xd6  |   -1   |   seconds in 32-bit unsigned int  |
                    // +--------+--------+--------+--------+--------+--------+
                    Timestamp::new(i64::from(BigEndian::read_u32(ext.data)), 0)
                }
                #[allow(clippy::cast_possible_wrap)]
                8 => {
                    // timestamp 64 stores the number of seconds and nanoseconds that have elapsed since 1970-01-01 00:00:00 UTC
                    // in 32-bit unsigned integers:
                    // +--------+--------+--------+--------+--------+------|-+--------+--------+--------+--------+
                    // |  0xd7  |   -1   | nanosec. in 30-bit unsigned int |   seconds in 34-bit unsigned int    |
                    // +--------+--------+--------+--------+--------+------^-+--------+--------+--------+--------+
                    let value = BigEndian::read_u64(ext.data);
                    Timestamp::new((value & 0x0000_0003_ffff_ffff_u64) as i64, (value >> 34) as u32)
                }
                #[cfg(feature = "timestamp96")]
                12 => {
                    // timestamp 96 stores the number of seconds and nanoseconds that have elapsed since 1970-01-01 00:00:00 UTC
                    // in 64-bit signed integer and 32-bit unsigned integer:
                    // +--------+--------+--------+--------+--------+--------+--------+
                    // |  0xc7  |   12   |   -1   |nanoseconds in 32-bit unsigned int |
                    // +--------+--------+--------+--------+--------+--------+--------+
                    // +--------+--------+--------+--------+--------+--------+--------+--------+
                    // |                   seconds in 64-bit signed int                        |
                    // +--------+--------+--------+--------+--------+--------+--------+--------+
                    let nanos = BigEndian::read_u32(&ext.data[0..4]);
                    let s = BigEndian::read_i64(&ext.data[4..12]);
                    Timestamp::new(s, nanos)
                }
                _ => Err(Error::InvalidType),
            }
        } else {
            Err(Error::InvalidType)
        }
    }
}

// #[cfg(feature = "std")]
// impl core::fmt::Debug for Timestamp {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         f.debug_struct("Timestamp")
//             .field("seconds", &self.seconds)
//             .field("nanoseconds", &self.nanoseconds)
//             .finish()
//     }
// }

// Does not make that much sense since it will be as big or bigger then Timestamp in memory...
pub struct TimestampRef<'a> {
    ext: Ext<'a>,
}

impl<'a> TryFrom<Ext<'a>> for TimestampRef<'a> {
    type Error = ();

    #[inline]
    fn try_from(ext: Ext<'a>) -> Result<Self, Self::Error> {
        if ext.typ == EXT_TIMESTAMP {
            Ok(TimestampRef { ext })
        } else {
            Err(())
        }
    }
}

impl<'a> From<TimestampRef<'a>> for Timestamp {
    #[inline]
    fn from(ts: TimestampRef<'a>) -> Self {
        match ts.ext.try_into() {
            Ok(x) => x,
            Err(_) => unreachable!(), // Unreachable, because TimestampRef can only be crated from valid Timestamps
        }
    }
}
