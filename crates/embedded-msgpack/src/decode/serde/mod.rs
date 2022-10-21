use crate::marker::Marker;
use core::fmt;
use paste::paste;
use serde::de::{self, Visitor};

use self::{enum_::UnitVariantAccess, map::MapAccess, seq::SeqAccess};

mod enum_;
mod map;
mod seq;

use super::Error;

type Result<T> = core::result::Result<T, Error>;

// #[cfg(test)]
fn print_debug<T>(prefix: &str, function_name: &str, de: &Deserializer) {
    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::println;
    println!(
        "{}{}<{}> ({:02x?})",
        prefix,
        function_name,
        core::any::type_name::<T>(),
        &de.slice[de.index..core::cmp::min(de.slice.len(), de.index + 10)]
    );
}
// #[cfg(test)]
fn print_debug_value<T, V: core::fmt::Debug>(function_name: &str, de: &Deserializer, value: &V) {
    #[cfg(not(feature = "std"))]
    extern crate std;
    #[cfg(not(feature = "std"))]
    use std::println;
    println!(
        "{}<{}> => {:?}   ({:02x?})",
        function_name,
        core::any::type_name::<T>(),
        value,
        &de.slice[de.index..core::cmp::min(de.slice.len(), de.index + 10)]
    );
}
// #[cfg(not(test))]
// fn print_debug<T>(_prefix: &str, _function_name: &str, _de: &Deserializer) {}
// #[cfg(not(test))]
// fn print_debug_value<T, V: core::fmt::Debug>(_function_name: &str, _de: &Deserializer, _value: &V) {}

pub(crate) struct Deserializer<'b> {
    slice: &'b [u8],
    index: usize,
}

impl<'a> Deserializer<'a> {
    pub const fn new(slice: &'a [u8]) -> Deserializer<'_> { Deserializer { slice, index: 0 } }

    fn eat_byte(&mut self) { self.index += 1; }

    fn peek(&mut self) -> Option<Marker> { Some(Marker::from_u8(*self.slice.get(self.index)?)) }
}

// NOTE(deserialize_*signed) we avoid parsing into u64 and then casting to a smaller integer, which
// is what upstream does, to avoid pulling in 64-bit compiler intrinsics, which waste a few KBs of
// Flash, when targeting non 64-bit architectures
macro_rules! deserialize_primitive {
    ($ty:ident) => {
        paste! {
            fn [<deserialize_ $ty>]<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value>
        {
            print_debug::<V>("Deserializer::deserialize_", stringify!($ty), &self);
            let (value, len) = paste! { super::[<read_ $ty>](&self.slice[self.index..])? };
            self.index += len;
            print_debug_value::<$ty, $ty>(stringify!(concat_idents!(Deserializer::deserialize_, $ty)), &self, &value);
            paste! { visitor.[<visit_ $ty>](value) }
        }}
    };
}
macro_rules! deserialize_primitives {
    ($($ty:ident),*) => { $( deserialize_primitive!($ty); )* };
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    deserialize_primitives!(bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "str", &self);
        let (s, len) = super::read_str(&self.slice[self.index..])?;
        self.index += len;
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "bytes", &self);
        let (value, len) = super::read_bin(&self.slice[self.index..])?;
        self.index += len;
        visitor.visit_borrowed_bytes(value)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "byte_buf", &self);
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "option", &self);
        let marker = self.peek().ok_or(Error::EndOfBuffer)?;
        match marker {
            Marker::Null => {
                self.eat_byte();
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "seq", &self);
        let (len, header_len) = crate::decode::read_array_len(&self.slice[self.index..])?;
        self.index += header_len;
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "tuple", &self);
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(self, _name: &'static str, _len: usize, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "tuple_struct", &self);
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "map", &self);
        let (len, header_len) = crate::decode::read_map_len(&self.slice[self.index..])?;
        self.index += header_len;
        visitor.visit_map(MapAccess::new(self, len))
    }

    fn deserialize_struct<V: Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "struct", &self);
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "enum", &self);
        visitor.visit_enum(UnitVariantAccess::new(self))
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "identifier", &self);
        self.deserialize_str(visitor)
    }

    /// Unsupported. Can’t parse a value without knowing its expected type.
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "any", &self);
        let (_, n) = super::skip_any(&self.slice[self.index..])?;
        self.index += n;
        visitor.visit_unit()
    }

    /// Used to throw out fields that we don’t want to keep in our structs.
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "ignored_any", &self);
        self.deserialize_any(visitor)
    }

    /// Unsupported. Use a more specific deserialize_* method
    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "unit", &self);
        let marker = self.peek().ok_or(Error::EndOfBuffer)?;
        match marker {
            Marker::Null | Marker::FixArray(0) => {
                self.eat_byte();
                visitor.visit_unit()
            }
            _ => Err(Error::InvalidType),
        }
    }

    /// Unsupported. Use a more specific deserialize_* method
    fn deserialize_unit_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "unit_struct", &self);
        self.deserialize_unit(visitor)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "char", &self);
        //TODO Need to decide how to encode this. Probably as a str?
        self.deserialize_str(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "newtype_struct", &self);
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        print_debug::<V>("Deserializer::deserialize_", "string", &self);
        self.deserialize_str(visitor)
    }
}

impl ::serde::de::StdError for Error {}
impl de::Error for Error {
    #[cfg_attr(not(feature = "custom-error-messages"), allow(unused_variables))]
    fn custom<T>(msg: T) -> Self
    where T: fmt::Display {
        #[cfg(not(feature = "custom-error-messages"))]
        {
            Error::CustomError
        }
        #[cfg(feature = "custom-error-messages")]
        {
            use core::fmt::Write;

            let mut string = heapless::String::new();
            write!(string, "{:.64}", msg).unwrap();
            Error::CustomErrorWithMessage(string)
        }
    }
}

impl fmt::Display for Error {
    #[cfg(debug_assertions)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidType => "Unexpected type encountered.",
                Error::OutOfBounds => "Index out of bounds.",
                Error::EndOfBuffer => "EOF while parsing.",
                Error::CustomError => "Did not match deserializer's expected format.",
                #[cfg(feature = "custom-error-messages")]
                Error::CustomErrorWithMessage(msg) => msg.as_str(),
                Error::NotAscii => "String contains non-ascii chars.",
                Error::InvalidBoolean => "Invalid boolean marker.",
                Error::InvalidBinType => "Invalid binary marker.",
                Error::InvalidStringType => "Invalid string marker.",
                Error::InvalidArrayType => "Invalid array marker.",
                Error::InvalidMapType => "Invalid map marker.",
                Error::InvalidNewTypeLength => "Invalid array length for newtype.",
            }
        )
    }
    #[cfg(not(debug_assertions))]
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result { Ok(()) }
}
