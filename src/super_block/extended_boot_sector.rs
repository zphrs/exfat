mod index;
extern crate alloc;
use std::io::{Read, Seek, Write};

use self::index::Index;
use bincode::{
    config::{
        AllowTrailing, FixintEncoding, LittleEndian, WithOtherEndian,
        WithOtherIntEncoding, WithOtherTrailing,
    },
    DefaultOptions, Options,
};
use serde::{de::DeserializeSeed, Serialize};

use crate::shift::BytesPerSector;

use super::extended_boot_code::{BytesPerSectorSeed, ExtendedBootCode};
pub trait Disk: Read + Write + Seek {}

impl<T> Disk for T where T: Read + Write + Seek {}
pub struct ExtendedBootSectors<'a, Disk>
where
    Disk: self::Disk,
{
    disk: &'a mut Disk,
    sector_size: BytesPerSector,
}
impl<'a, Disk> ExtendedBootSectors<'a, Disk>
where
    Disk: self::Disk,
{
    // only run on initial formatting
    pub fn init(disk: &'a mut Disk, sector_size: BytesPerSector) -> Self {
        let mut out = Self::open(disk, sector_size);
        for index in (0..7).map(|i| Index::new(i).unwrap()) {
            out.set_sector_code(index, &[]).unwrap();
        }

        out
    }

    pub fn open(disk: &'a mut Disk, sector_size: BytesPerSector) -> Self {
        Self { disk, sector_size }
    }

    fn offset_from_index(&self, index: Index) -> u64 {
        (*self.sector_size + *self.sector_size * (*index as usize)) as u64
    }
    // index should be some number between 0 to 7
    pub fn load_sector(
        &mut self,
        index: Index,
    ) -> Result<ExtendedBootCode, Box<bincode::ErrorKind>> {
        self.disk
            .seek(std::io::SeekFrom::Start(self.offset_from_index(index)))?;
        let my_options = bincode::DefaultOptions::new()
            .allow_trailing_bytes()
            .with_fixint_encoding()
            .with_little_endian();
        my_options.deserialize_from_seed(
            BytesPerSectorSeed(self.sector_size),
            &mut self.disk,
        )
    }

    pub fn set_sector_code(
        &mut self,
        index: Index,
        code: &[u8],
    ) -> Result<(), Box<bincode::ErrorKind>> {
        self.disk
            .seek(std::io::SeekFrom::Start(self.offset_from_index(index)))?;
        let bc = ExtendedBootCode::new(code, self.sector_size);
        let my_options = bincode::DefaultOptions::new()
            .allow_trailing_bytes()
            .with_fixint_encoding()
            .with_little_endian();
        my_options.serialize_into(&mut self.disk, &bc)?;
        Ok(())
    }
}
