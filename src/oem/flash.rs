use serde::{
    ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer,
};
use uguid::{guid, Guid};

use super::Parameter;

pub(super) const fn flash_guid() -> Guid {
    guid!("0A0C7E46-3399-4021-90C8-FA6D389C4BA2")
}

// https://learn.microsoft.com/en-us/windows/win32/fileio/exfat-specification#334-flash-parameters
#[derive(Serialize, Deserialize)]
pub struct FlashParameter {
    /// ## Description
    ///
    /// The ParametersGuid field shall conform to the definition provided in
    /// the Generic Parameters template (see Section 3.3.2.1).
    ///
    /// ## Value
    ///
    /// The valid value for this field, in GUID notation, is
    /// {0A0C7E46-3399-4021-90C8-FA6D389C4BA2}.
    #[serde(default = "flash_guid")]
    // I skip serde here because in my implementation of Parameter I handle
    // the serialization, deserialization, and verification of the guid.
    // Aka if it gets to deserializing the flash parameter then I have already
    // ensured that the deserialized guid == flash_guid;
    #[serde(skip)]
    _parameters_guid: Guid,
    ///The EraseBlockSize field shall describe the size, in bytes, of the
    /// flash media's erase block.
    erase_block_size: u32,
    /// The PageSize field shall describe the size, in bytes of the flash
    /// media's page.
    page_size: u32,
    /// The SpareSectors field shall describe the number of sectors the flash
    /// media has available for its internal sparing operations.
    spare_sectors: u32,
    /// The RandomAccessTime field shall describe the flash media's average
    /// random access time, in nanoseconds.
    random_access_time: u32,
    /// The ProgrammingTime field shall describe the flash media's average
    /// programming time, in nanoseconds.
    programming_time: u32,
    /// The ReadCycle field shall describe the flash media's average read cycle
    /// time, in nanoseconds.
    read_cycle: u32,
    /// The WriteCycle field shall describe the average write cycle time, in
    /// nanoseconds.
    write_cycle: u32,
}
#[derive(Default)]
pub struct FlashOptions {
    /// The EraseBlockSize field shall describe the size, in bytes, of the
    /// flash media's erase block.
    pub erase_block_size: u32,
    /// The PageSize field shall describe the size, in bytes of the flash
    /// media's page.
    pub page_size: u32,
    /// The SpareSectors field shall describe the number of sectors the flash
    /// media has available for its internal sparing operations.
    pub spare_sectors: u32,
    /// The RandomAccessTime field shall describe the flash media's average
    /// random access time, in nanoseconds.
    pub random_access_time: u32,
    /// The ProgrammingTime field shall describe the flash media's average
    /// programming time, in nanoseconds.
    pub programming_time: u32,
    /// The ReadCycle field shall describe the flash media's average read cycle
    /// time, in nanoseconds.
    pub read_cycle: u32,
    /// The WriteCycle field shall describe the average write cycle time, in
    /// nanoseconds.
    pub write_cycle: u32,
}

impl From<FlashOptions> for FlashParameter {
    fn from(value: FlashOptions) -> Self {
        let FlashOptions {
            erase_block_size,
            page_size,
            spare_sectors,
            random_access_time,
            programming_time,
            read_cycle,
            write_cycle,
        } = value;
        Self {
            _parameters_guid: flash_guid(),
            erase_block_size,
            page_size,
            spare_sectors,
            random_access_time,
            programming_time,
            read_cycle,
            write_cycle,
        }
    }
}

impl FlashParameter {
    pub fn new(options: FlashOptions) -> FlashParameter {
        options.into()
    }
}

impl From<FlashParameter> for Parameter {
    fn from(value: FlashParameter) -> Self {
        Self::Flash(value)
    }
}
