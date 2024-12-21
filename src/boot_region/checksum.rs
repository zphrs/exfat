use crate::shift::BytesPerSector;

mod de;
mod ser;

pub struct Checksum {
    checksum: u32,
    sector_size: usize,
}

impl Checksum {
    pub fn new(checksum: u32, sector_size: BytesPerSector) -> Self {
        Self {
            checksum,
            sector_size: *sector_size,
        }
    }
}
