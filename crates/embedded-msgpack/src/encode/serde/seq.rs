use serde::ser;

use super::{Error, Serializer};

pub struct SerializeSeq<'a, 'b> {
    ser: &'a mut Serializer<'b>,
}

impl<'a, 'b> SerializeSeq<'a, 'b> {
    pub(crate) fn new(ser: &'a mut Serializer<'b>) -> Self { SerializeSeq { ser } }
}

impl<'a, 'b> ser::SerializeSeq for SerializeSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where T: ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
}

impl<'a, 'b> ser::SerializeTuple for SerializeSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where T: ser::Serialize {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { ser::SerializeSeq::end(self) }
}
