use serde::ser;

use heapless::ArrayLength;

use crate::ser::{BTError, Result, Serializer};

pub struct SerializeStruct<'a, B>
where
    B: ArrayLength<u8>,
{
    de: &'a mut Serializer<B>,
    first: bool,
}

impl<'a, B> SerializeStruct<'a, B>
where
    B: ArrayLength<u8>,
{
    pub(crate) fn new(de: &'a mut Serializer<B>) -> Self {
        SerializeStruct { de, first: true }
    }
}

impl<'a, B> ser::SerializeStruct for SerializeStruct<'a, B>
where
    B: ArrayLength<u8>,
{
    type Ok = ();
    type Error = BTError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        // XXX if `value` is `None` we not produce any output for this field
        if !self.first {
            self.de.buf.push(b',')?;
        }
        self.first = false;

        self.de.buf.push(b'"')?;
        self.de.buf.extend_from_slice(key.as_bytes())?;
        self.de.buf.extend_from_slice(b"\":")?;

        value.serialize(&mut *self.de)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.de.buf.push(b'}')?;
        Ok(())
    }
}
