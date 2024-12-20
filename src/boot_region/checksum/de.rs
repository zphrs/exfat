use std::ops::Deref;

use serde::de::DeserializeSeed;

use crate::shift::BytesPerSector;

use super::Checksum;

pub struct BytesPerSectorSeed(pub BytesPerSector);

impl Deref for BytesPerSectorSeed {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &(*self.0)
    }
}

impl<'de> DeserializeSeed<'de> for BytesPerSectorSeed {
    type Value = Checksum;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let sector_len = *self;
        struct Visitor(usize);
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Checksum;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(
                    formatter,
                    "A repeating series of identical u32 checksums that \
                    fully fills the sector."
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let checksum: u32 = seq.next_element()?.ok_or(
                    serde::de::Error::invalid_length(
                        self.0,
                        &"a repeating series of checksums",
                    ),
                )?;
                for _ in 0..self.0 {
                    let continued_checksum: u32 = seq.next_element()?.ok_or(
                        serde::de::Error::invalid_length(
                            self.0,
                            &"a repeating series of checksums",
                        ),
                    )?;
                    if continued_checksum != checksum {
                        return Err(serde::de::Error::custom(
                            "Checksum should be consistent \
                            throughout the sector.",
                        ));
                    }
                }
                Ok(Checksum {
                    checksum,
                    sector_size: self.0,
                })
            }
        }
        deserializer.deserialize_tuple(
            sector_len / size_of::<u32>(),
            Visitor(sector_len),
        )
    }
}
