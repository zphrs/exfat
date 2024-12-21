extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
mod de;
mod ser;
pub use de::BytesPerSectorSeed;

use crate::shift::BytesPerSector;

const VALID_SIGNATURE: u32 = 0xAA550000;

#[derive(Clone, PartialEq, Eq)]
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

impl ExtendedBootCode {
    pub fn new(boot_code: &[u8], sector_size: BytesPerSector) -> Self {
        if *sector_size < 4 {
            panic!("There must be at least 4 bytes per sector to fit the signature.")
        }
        // assert length of boot_code is not too long
        if boot_code.len() > (*sector_size - 4) {
            panic!("Boot code is too long!");
        }
        let mut bc = vec![0x00; *sector_size - 4];
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

    use crate::{
        shift::ShiftedBytes,
        super_block::extended_boot_code::BytesPerSectorSeed,
    };

    use super::{BytesPerSector, ExtendedBootCode};

    #[test]
    fn serde() {
        const BYTES_PER_SECTOR: BytesPerSector = ShiftedBytes::new(3).unshift();
        let ebs = ExtendedBootCode::new(b"code", BYTES_PER_SECTOR);
        let out: Vec<u8> = Vec::with_capacity(*BYTES_PER_SECTOR);
        let mut cursor = std::io::Cursor::new(out);

        let bincode_options = bincode::DefaultOptions::new()
            .allow_trailing_bytes()
            .with_little_endian()
            .with_fixint_encoding();
        bincode_options.serialize_into(&mut cursor, &ebs).unwrap();
        cursor.set_position(0);
        let deserialized = bincode_options
            .deserialize_from_seed(BytesPerSectorSeed(BYTES_PER_SECTOR), cursor)
            .unwrap();

        assert!(deserialized == ebs);
    }
}
