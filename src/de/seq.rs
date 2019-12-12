use serde::de;

use crate::de::{Deserializer, BTError, Result};

pub(crate) struct SeqAccess<'a, 'b> {
    first: bool,
    de: &'a mut Deserializer<'b>,
}

impl<'a, 'b> SeqAccess<'a, 'b> {
    pub fn new(de: &'a mut Deserializer<'b>) -> Self {
        SeqAccess { de, first: true }
    }
}

impl<'a, 'de> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = BTError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        let peek = match self
            .de
            .parse_whitespace()
            .ok_or(BTError::EofWhileParsingList)?
        {
            b']' => return Ok(None),
            b',' => {
                self.de.eat_char();
                self.de
                    .parse_whitespace()
                    .ok_or(BTError::EofWhileParsingValue)?
            }
            c => {
                if self.first {
                    self.first = false;
                    c
                } else {
                    return Err(BTError::ExpectedListCommaOrEnd);
                }
            }
        };

        if peek == b']' {
            Err(BTError::TrailingComma)
        } else {
            Ok(Some(seed.deserialize(&mut *self.de)?))
        }
    }
}
