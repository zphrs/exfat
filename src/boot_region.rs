use serde::{de::DeserializeSeed, Deserialize, Serialize};

use crate::boot_sector::{extended_boot_code::ExtendedBootCode, BootSector};



struct BootRegion {
    boot_sector: BootSector,
    extended_boot_sector: ExtendedBootCode,
}
