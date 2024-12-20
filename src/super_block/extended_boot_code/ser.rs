use serde::{ser::SerializeTuple, Serialize};

use super::ExtendedBootCode;

impl Serialize for ExtendedBootCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(self.code.len() + 1)?;
        for byte in &self.code {
            seq.serialize_element(&byte)?;
        }
        seq.serialize_element(&self.signature)?;
        seq.end()
    }
}
