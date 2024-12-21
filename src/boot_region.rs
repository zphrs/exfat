mod checksum;

use std::io::{Read, Seek, SeekFrom, Write};

use checksum::Checksum;

use crate::{
    oem::Oem,
    shift::{BytesPerSector, SectorsPerCluster},
    super_block::{
        boot_code::BootCode, extended_boot_code::ExtendedBootCode, SuperBlock,
    },
};

struct BootRegion<Disk> {
    // #[sectors(0..1)]
    boot_sector: SuperBlock,
    // #[sectors(1..9)]
    extended_boot_code: [ExtendedBootCode; 8],
    // #[sectors(9..10)]
    oem: Oem,
    // #[sectors(10..11)]
    _reserved: (),
    // #[sectors(11..12)]
    boot_checksum: Checksum,
    disk: Disk,
}

impl<Disk: Read + Write + Seek> BootRegion<Disk> {
    pub fn format(
        bytes_per_sector: BytesPerSector,
        sectors_per_cluster: SectorsPerCluster,
        boot_code: BootCode,
        extended_boot_code: [ExtendedBootCode; 8],
        volume_length: u64,
        oem: Oem,
        disk: Disk,
    ) -> Self {
        println!("Making superblock.");
        let boot_sector = SuperBlock::new(
            bytes_per_sector,
            sectors_per_cluster,
            boot_code,
            volume_length,
        );
        let bytes_per_sector = boot_sector.bytes_per_sector();
        println!("Making boot region.");
        let mut out = BootRegion {
            boot_sector,
            extended_boot_code,
            oem,
            _reserved: (),
            disk,
            boot_checksum: Checksum::new(0, bytes_per_sector),
        };
        println!("Calculating checksum");
        let checksum = out.calculate_checksum();
        out.boot_checksum = Checksum::new(checksum, bytes_per_sector);
        out.update_disk_checksum();
        out
    }

    pub fn bytes_per_sector(&self) -> BytesPerSector {
        self.boot_sector.bytes_per_sector()
    }

    pub fn calculate_checksum(&mut self) -> u32 {
        let mut checksum = 0u32;
        self.update_disk_without_checksum();
        self.disk.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = vec![0u8; *self.bytes_per_sector() * 11];
        self.disk.read_exact(buf.as_mut_slice()).unwrap();

        for (i, byte) in buf.iter().enumerate() {
            if [
                106, 107, // volumeFlags
                112, // percentInUse
            ]
            .contains(&i)
            {
                continue;
            }
            checksum = checksum.rotate_right(1) + (*byte as u32);
        }
        checksum
    }
    fn update_disk_without_checksum(&mut self) {
        // vec implements write; appends to end
        let mut d = &mut self.disk;
        d.seek(SeekFrom::Start(0)).unwrap();
        println!("Writing boot sector");
        bincode::serialize_into(&mut d, &self.boot_sector).unwrap();
        println!("Writing extended boot code");
        bincode::serialize_into(&mut d, &self.extended_boot_code).unwrap();
        println!("Writing oem");
        bincode::serialize_into(&mut d, &self.oem).unwrap();
        // one reserved empty sector
        println!("Writing empty sector");
        d.write_all(
            (0..*self.boot_sector.bytes_per_sector())
                .map(|_| 0u8)
                .collect::<Vec<u8>>()
                .as_ref(),
        )
        .unwrap();
    }

    pub fn update_disk_checksum(&mut self) {
        let start = *self.bytes_per_sector() as u64 * 11;
        let d = &mut self.disk;
        d.seek(SeekFrom::Start(start)).unwrap();
        bincode::serialize_into(d, &self.boot_checksum).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        oem::Oem,
        shift::{BytesPerSector, ShiftedBytes, ShiftedSectors},
        super_block::{
            boot_code::BootCode, extended_boot_code::ExtendedBootCode,
        },
    };

    use super::BootRegion;

    #[test]
    pub fn test_format() {
        const DISK_SIZE: usize = 2usize.pow(25);
        let bytes_per_sector: BytesPerSector = ShiftedBytes::new(12).unshift();
        let mut emulated_disk = vec![0u8; *bytes_per_sector * 12];
        let disk = Cursor::new(&mut emulated_disk);
        BootRegion::format(
            bytes_per_sector,
            ShiftedSectors::from(8).into(),
            BootCode::new(&[]),
            core::array::from_fn(|_i| {
                ExtendedBootCode::new(&[], bytes_per_sector)
            }),
            DISK_SIZE as u64,
            Oem::new(),
            disk,
        );
        let no_zeros = emulated_disk.chunk_by(|a, b| a == b).map(|n| {
            if (n)[0] != 0 || n.len() < 32 {
                format!(
                    "{}",
                    n.iter()
                        .map(|n| format!("{}", n))
                        .collect::<Vec<String>>()
                        .join("")
                )
            } else {
                format!("\n({} zeroes)\n", n.len())
            }
        });
        for byte in no_zeros {
            print!("{byte}")
        }
        println!("");
        panic!();
    }
}
