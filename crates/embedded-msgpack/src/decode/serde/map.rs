use serde::de;

use super::{print_debug, Deserializer, Error};

pub struct MapAccess<'a, 'b> {
    de: &'a mut Deserializer<'b>,
    count: usize,
}

impl<'a, 'b> MapAccess<'a, 'b> {
    pub(crate) fn new(de: &'a mut Deserializer<'b>, count: usize) -> Self { MapAccess { de, count: count * 2 } }
}

impl<'a, 'de> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where K: de::DeserializeSeed<'de> {
        print_debug::<K>("MapAccess::", "next_key_seed", &self.de);
        if self.count > 0 {
            self.count -= 1;
            Ok(Some(seed.deserialize(&mut *self.de)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where V: de::DeserializeSeed<'de> {
        print_debug::<V>("MapAccess::", "next_value_seed", &self.de);
        if self.count > 0 {
            self.count -= 1;
            Ok(seed.deserialize(&mut *self.de)?)
        } else {
            Err(Error::EndOfBuffer)
        }
    }
}
