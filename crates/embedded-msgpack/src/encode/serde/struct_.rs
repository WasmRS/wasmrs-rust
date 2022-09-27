use serde::ser::{self, Serialize};

use super::{Error, Serializer};

pub struct SerializeStruct<'a, 'b> {
    ser: &'a mut Serializer<'b>,
}

impl<'a, 'b> SerializeStruct<'a, 'b> {
    pub(crate) fn new(ser: &'a mut Serializer<'b>) -> Self { SerializeStruct { ser } }
}

impl<'a, 'b> ser::SerializeStruct for SerializeStruct<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where T: ser::Serialize {
        key.serialize(&mut *self.ser)?;
        value.serialize(&mut *self.ser)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
}
