use crate::{
    oem::Oem,
    super_block::{boot_code::BootCode, SuperBlock},
};

pub struct FileSystem {
    // #[sectors(0)]
    main_boot_sector: SuperBlock,
    // #[sectors(0)]
    backup_boot_sector: SuperBlock,
    // #[sectors(0)]
    oem_parameters: Oem,
    reserved: (),
}

impl FileSystem {}
