use serde::ser;

// use heapless::ArrayLength;

use super::{Error, Serializer};

pub struct SerializeMap<'a, 'b> {
    ser: &'a mut Serializer<'b>,
}

impl<'a, 'b> SerializeMap<'a, 'b> {
    pub(crate) fn new(ser: &'a mut Serializer<'b>) -> Self { SerializeMap { ser } }
}

impl<'a, 'b> ser::SerializeMap for SerializeMap<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where T: ser::Serialize {
        key.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where T: ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }
}
