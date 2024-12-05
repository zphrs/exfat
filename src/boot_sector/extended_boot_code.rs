use macros::DiskLayout;
use serde::{
    de::{DeserializeSeed, SeqAccess},
    ser::SerializeTuple,
    Serialize,
};

use crate::shift::BytesPerSector;

const VALID_SIGNATURE: u32 = 0xAA550000;

#[derive(DiskLayout, Clone, PartialEq, Eq)]
pub struct ExtendedBootCode {
    /// ## Description
    /// The ExtendedBootCode field shall contain boot-strapping instructions.
    /// Implementations may populate this field with the CPU instructions
    /// necessary for boot-strapping a computer system. Implementations which
    /// don't provide boot-strapping instructions shall initialize each byte in
    /// this field to 00h as part of their format operation.
    code: Vec<u8>,
    /// ## Description
    /// The BootSignature field shall describe whether the intent of a given
    /// sector is for it to be a Boot Sector or not.
    /// ## Value
    /// The valid value for this field is AA55h. Any other value in this field
    /// invalidates its respective Boot Sector. Implementations should verify
    /// the contents of this field prior to depending on any other field in its
    /// respective Boot Sector.
    signature: u32,
}

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

impl<'de> DeserializeSeed<'de> for BytesPerSector {
    type Value = ExtendedBootCode;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor(usize);

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExtendedBootCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a tuple of bytes of length bytes_per_sector - 4 \
                followed by a single element of the tuple of u32."
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<ExtendedBootCode, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut bc = vec![0x00u8; self.0 - 4];

                for byte in bc.iter_mut() {
                    *byte = seq.next_element()?.ok_or(serde::de::Error::invalid_length(
                        self.0 - 4,
                        &"a tuple of bytes of length bytes_per_sector - 4 was expected",
                    ))?;
                }
                let signature: u32 = seq
                    .next_element()?
                    .ok_or(serde::de::Error::missing_field("signature"))?;
                // check that signature is valid
                if signature != VALID_SIGNATURE {
                    return Err(serde::de::Error::custom(
                        "Signature is invalid. This boot code is corrupted.",
                    ));
                }
                Ok(ExtendedBootCode {
                    code: bc,
                    signature,
                })
            }
        }
        deserializer.deserialize_tuple(*self - 4 + 1, Visitor(*self))
    }
}

impl ExtendedBootCode {
    pub fn new(boot_code: &[u8], bytes_per_sector: usize) -> Self {
        if bytes_per_sector < 4 {
            panic!("There must be at least 4 bytes per sector to fit the signature.")
        }
        // assert length of boot_code is not too long
        if boot_code.len() > (bytes_per_sector - 4) {
            panic!("Boot code is too long!");
        }
        let mut bc = vec![0x00; bytes_per_sector - 4];
        bc[0..boot_code.len()].copy_from_slice(boot_code);
        Self {
            code: bc,
            signature: VALID_SIGNATURE,
        }
    }
}
#[cfg(test)]
mod tests {

    use bincode::Options;

    use crate::shift::ShiftedBytes;

    use super::{BytesPerSector, ExtendedBootCode};

    #[test]
    fn serde() {
        const BYTES_PER_SECTOR: BytesPerSector = ShiftedBytes::new(3).unshift();
        let ebs = ExtendedBootCode::new(b"code", *BYTES_PER_SECTOR);
        let out: Vec<u8> = Vec::with_capacity(*BYTES_PER_SECTOR);
        let mut cursor = std::io::Cursor::new(out);

        let bincode_options = bincode::DefaultOptions::new()
            .allow_trailing_bytes()
            .with_little_endian()
            .with_fixint_encoding();
        bincode_options.serialize_into(&mut cursor, &ebs).unwrap();
        cursor.set_position(0);
        let deserialized = bincode_options
            .deserialize_from_seed(BYTES_PER_SECTOR, cursor)
            .unwrap();

        assert!(deserialized == ebs);
    }
}
