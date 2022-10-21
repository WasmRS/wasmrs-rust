use serde::de;

use super::{print_debug, Deserializer, Error};

pub(crate) struct UnitVariantAccess<'a, 'b> {
    de: &'a mut Deserializer<'b>,
}

impl<'a, 'b> UnitVariantAccess<'a, 'b> {
    pub(crate) fn new(de: &'a mut Deserializer<'b>) -> Self { UnitVariantAccess { de } }
}

impl<'a, 'de> de::EnumAccess<'de> for UnitVariantAccess<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self), Self::Error>
    where V: de::DeserializeSeed<'de> {
        print_debug::<V>("UnitVariantAccess::", "variant_seed", &self.de);
        let variant = seed.deserialize(&mut *self.de)?;
        Ok((variant, self))
    }
}

impl<'de, 'a> de::VariantAccess<'de> for UnitVariantAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        print_debug::<()>("UnitVariantAccess::", "unit_variant", &self.de);
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where T: de::DeserializeSeed<'de> {
        print_debug::<T>("UnitVariantAccess::", "newtype_variant_seed", &self.de);

        let (len, header_len) = crate::decode::read_array_len(&self.de.slice[self.de.index..])?;
        self.de.index += header_len;
        match len {
            1 => seed.deserialize(self.de),
            0 => Err(Error::InvalidNewTypeLength),
            _ => Err(Error::InvalidNewTypeLength),
        }
    }

    fn tuple_variant<V>(self, v_len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where V: de::Visitor<'de> {
        print_debug::<V>("UnitVariantAccess::", "tuple_variant", &self.de);
        let (len, header_len) = crate::decode::read_array_len(&self.de.slice[self.de.index..])?;
        if len != v_len {
            return Err(Error::OutOfBounds);
        }
        self.de.index += header_len;
        visitor.visit_seq(super::SeqAccess::new(&mut *self.de, len))
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where V: de::Visitor<'de> {
        print_debug::<V>("UnitVariantAccess::", "struct_variant", &self.de);
        let (len, header_len) = crate::decode::read_map_len(&self.de.slice[self.de.index..])?;
        self.de.index += header_len;
        if len != fields.len() {
            return Err(Error::OutOfBounds);
        }
        visitor.visit_map(super::MapAccess::new(&mut *self.de, fields.len()))
    }
}
