mod index;

use std::io::{Read, Seek, Write};

use self::index::Index;
use serde::{de::DeserializeSeed, Serialize};

use crate::shift::BytesPerSector;
trait Disk: Read + Write + Seek {}

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
    pub fn init(disk: &'a mut Disk, sector_size: BytesPerSector) -> Self {
        Self { disk, sector_size }
    }
    // index should be some number between 1 to 8
    pub fn load_sector(&self, index: Index) -> Result<u64, std::io::Error> {
        self.disk.seek(std::io::SeekFrom::Start(
            (*self.sector_size + *self.sector_size * (*index as usize)) as u64,
        ))
    }
}
