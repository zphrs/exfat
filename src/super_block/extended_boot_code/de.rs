use std::ops::Deref;

use serde::de::{DeserializeSeed, SeqAccess};

use crate::shift::BytesPerSector;

use super::{ExtendedBootCode, VALID_SIGNATURE};

pub struct BytesPerSectorSeed(pub BytesPerSector);

impl Deref for BytesPerSectorSeed {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &(*self.0)
    }
}

impl<'de> DeserializeSeed<'de> for BytesPerSectorSeed {
    type Value = ExtendedBootCode;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor(usize);

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExtendedBootCode;

            fn expecting(
                &self,
                formatter: &mut core::fmt::Formatter,
            ) -> core::fmt::Result {
                write!(
                    formatter,
                    "A tuple of two elements: \n\
                    - A tuple of u8 with \
                      length bytes_per_sector - 4 \n\
                    - a u32."
                )
            }

            fn visit_seq<A>(
                self,
                mut seq: A,
            ) -> Result<ExtendedBootCode, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut boot_code = vec![0x00u8; self.0 - 4];

                for byte in boot_code.iter_mut() {
                    *byte = seq.next_element()?.ok_or(
                        serde::de::Error::invalid_length(
                            self.0 - 4,
                            &"a tuple of bytes of length \
                        bytes_per_sector - 4 was expected",
                        ),
                    )?;
                }
                let signature: u32 = seq
                    .next_element()?
                    .ok_or(serde::de::Error::missing_field("signature"))?;
                // check that signature is valid
                if signature != VALID_SIGNATURE {
                    return Err(serde::de::Error::custom(
                        "Signature is invalid. \
                        This boot code is corrupted.",
                    ));
                }
                Ok(ExtendedBootCode {
                    code: boot_code,
                    signature,
                })
            }
        }
        deserializer.deserialize_tuple(*self - 4 + 1, Visitor(*self))
    }
}
