#[cfg(feature = "timestamp")]
pub mod timestamp;

use crate::encode::{Error, SerializeIntoSlice};
#[allow(unused_imports)]
use crate::marker::Marker;

#[allow(unused_imports)]
use byteorder::{BigEndian, ByteOrder};

pub struct Ext<'a> {
    typ: i8,
    data: &'a [u8],
}

impl<'a> Ext<'a> {
    pub const fn new(typ: i8, data: &'a [u8]) -> Self { Ext { typ, data } }
    #[inline(always)]
    pub const fn get_type(&self) -> i8 { self.typ }
    #[inline(always)]
    pub const fn get_data(&self) -> &'a [u8] { self.data }
}

pub fn serialize_ext<'a>(value: &Ext<'a>, buf: &mut [u8]) -> Result<usize, Error> {
    let typ = value.get_type();
    let data = value.get_data();

    let (marker, header_len) = match data.len() {
        #[cfg(feature = "fixext")]
        1 | 2 | 4 | 8 | 16 => {
            let header_len = 2;
            let marker = match data.len() {
                1 => Marker::FixExt1.to_u8(),
                2 => Marker::FixExt2.to_u8(),
                4 => Marker::FixExt4.to_u8(),
                8 => Marker::FixExt8.to_u8(),
                16 => Marker::FixExt16.to_u8(),
                _ => unreachable!(),
            };
            // buf[0] = marker;
            // buf[1] = typ as u8;
            // buf[header_len..data.len() + header_len].clone_from_slice(data);
            // return Ok(data.len() + header_len);

            (marker, header_len)
        }
        #[cfg(feature = "ext8")]
        0..=0xff => {
            let header_len = 3;
            // if buf.len() < data.len() + header_len {
            //     return Err(Error::EndOfBuffer);
            // }
            // buf[0] = Marker::Ext8.to_u8();
            // buf[1] = data.len() as u8;
            // buf[header_len - 1] = typ as u8;
            // buf[header_len..data.len() + header_len].clone_from_slice(data);
            // return Ok(data.len() + header_len);

            (Marker::Ext8.to_u8(), header_len)
        }
        #[cfg(feature = "ext16")]
        0x100..=0xffff => {
            let header_len = 4;
            // if buf.len() < data.len() + header_len {
            //     return Err(Error::EndOfBuffer);
            // }
            // buf[0] = Marker::Ext16.to_u8();
            // BigEndian::write_u16(&mut buf[1..], data.len() as u16);
            // buf[header_len - 1] = typ as u8;
            // buf[header_len..data.len() + header_len].clone_from_slice(data);
            // return Ok(data.len() + header_len);

            (Marker::Ext16.to_u8(), header_len)
        }
        #[cfg(feature = "ext32")]
        0x1_0000..=0xffff_ffff => {
            let header_len = 6;
            // if buf.len() < data.len() + header_len {
            //     return Err(Error::EndOfBuffer);
            // }
            // buf[0] = Marker::Ext32.to_u8();
            // BigEndian::write_u32(&mut buf[1..], data.len() as u32);
            // buf[header_len - 1] = typ as u8;
            // buf[header_len..data.len() + header_len].clone_from_slice(data);
            // return Ok(data.len() + header_len);

            (Marker::Ext32.to_u8(), header_len)
        }
        _ => {
            // if data.len().is_power_of_two() {
            //     (Marker::FixExt1.to_u8(), 2)
            // } else if data.len() <= u8::max_value() as usize {
            //     (Marker::Ext8.to_u8(), 3)
            // } else if data.len() <= u16::max_value() as usize {
            //     (Marker::Ext16.to_u8(), 4)
            // } else {
            //     (Marker::Ext32.to_u8(), 6)
            // }
            return Err(Error::InvalidType);
        }
    };
    if buf.len() < data.len() + header_len {
        return Err(Error::EndOfBuffer);
    }
    buf[0] = marker;
    // match data.len() {
    //     0..=0xff => buf[1] = data.len() as u8,
    //     0x100..=0xffff => BigEndian::write_u16(&mut buf[1..], data.len() as u16),
    //     0x10000..=0xffffffff => BigEndian::write_u32(&mut buf[1..], data.len() as u32),
    //     _ => unreachable!(),
    // };
    if header_len > 2 {
        #[cfg(all(feature = "ext8", not(any(feature = "ext16", feature = "ext32"))))]
        {
            buf[1] = data.len() as u8;
        }
        #[cfg(any(feature = "ext16", feature = "ext32"))]
        {
            BigEndian::write_uint(&mut buf[1..], data.len() as u64, header_len - 2);
        }
    }
    buf[header_len - 1] = typ as u8;
    buf[header_len..data.len() + header_len].clone_from_slice(data);
    Ok(data.len() + header_len)
}

#[cfg(feature = "ext")]
impl<'a> SerializeIntoSlice for &Ext<'a> {
    fn write_into_slice(&self, buf: &mut [u8]) -> Result<usize, Error> { serialize_ext(self, buf) }
}
