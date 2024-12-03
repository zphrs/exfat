use crate::boot_sector::{boot_code::BootCode, BootSector};

pub struct FileSystem {
    main_boot_sector: BootSector,
    backup_boot_sector: BootSector,
}

impl FileSystem {
    pub fn new(
        bytes_per_sector_shift: u8,
        sectors_per_cluster_shift: u8,
        cluster_heap_offset: u32,
        boot_code: BootCode,
        backup_boot_code: BootCode,
    ) -> Self {
        let main_boot_sector = BootSector::new(
            bytes_per_sector_shift,
            sectors_per_cluster_shift,
            cluster_heap_offset,
            boot_code,
        );
        let mut backup_boot_sector = main_boot_sector.clone();
        backup_boot_sector.set_boot_code(backup_boot_code);
        FileSystem {
            main_boot_sector,
            backup_boot_sector,
        }
    }
}
