use crate::{
    oem::Oem,
    super_block::{boot_code::BootCode, SuperBlock},
};

pub struct FileSystem {
    // #[sectors(0)]
    main_boot_sector: SuperBlock,
    backup_boot_sector: SuperBlock,
    // #[sectors(0)]
    oem_parameters: Oem,
    reserved: (),
}

impl FileSystem {
    pub fn format(
        bytes_per_sector_shift: u8,
        sectors_per_cluster_shift: u8,
        boot_code: BootCode,
        backup_boot_code: BootCode,
        volume_length: u64,
        oem_parameters: Oem,
    ) -> Self {
        let main_boot_sector = SuperBlock::new(
            bytes_per_sector_shift,
            sectors_per_cluster_shift,
            boot_code,
            volume_length,
        );
        let mut backup_boot_sector = main_boot_sector.clone();
        backup_boot_sector.set_boot_code(backup_boot_code);
        FileSystem {
            main_boot_sector,
            backup_boot_sector,
            oem_parameters: oem,
        }
    }
}
