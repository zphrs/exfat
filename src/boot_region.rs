mod checksum;

use checksum::Checksum;

use crate::{
    oem::Oem,
    super_block::{extended_boot_code::ExtendedBootCode, SuperBlock},
};

struct BootRegion {
    boot_sector: SuperBlock,
    extended_boot_sector: ExtendedBootCode,
    oem: Oem,
    reserved: (),
    boot_checksum: Checksum,
}
