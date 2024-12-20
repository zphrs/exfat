mod ser;
mod de;

pub struct Checksum {
    checksum: u32,
    sector_size: usize,
}

