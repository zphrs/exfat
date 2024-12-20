use serde::{ser::SerializeTuple, Serialize};

use super::Checksum;

impl Checksum {
    pub fn repeat_count(&self) -> usize {
        self.sector_size / size_of::<u32>()
    }
}

impl Serialize for Checksum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(self.repeat_count())?;
        for _ in 0..self.repeat_count() {
            seq.serialize_element(&self.checksum)?;
        }
        seq.end()
    }
}
