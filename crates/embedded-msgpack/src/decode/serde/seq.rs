use serde::de;

use super::{print_debug, Deserializer, Error};

pub(crate) struct SeqAccess<'a, 'b> {
    de: &'a mut Deserializer<'b>,
    count: usize,
}

impl<'a, 'b> SeqAccess<'a, 'b> {
    pub fn new(de: &'a mut Deserializer<'b>, count: usize) -> Self { SeqAccess { de, count } }
}

impl<'a, 'de> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where T: de::DeserializeSeed<'de> {
        print_debug::<T>("SeqAccess::", "next_element_seed", &self.de);
        if self.count > 0 {
            self.count -= 1;
            Ok(Some(seed.deserialize(&mut *self.de)?))
        } else {
            Ok(None)
        }
    }
}
