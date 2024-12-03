// 3.3 - https://learn.microsoft.com/en-us/windows/win32/fileio/exfat-specification#33-main-and-backup-oem-parameters-sub-regions

use serde::{de::Visitor, ser::SerializeTuple, Deserialize, Serialize};
use serde_big_array::BigArray;
use uguid::{guid, Guid};

const fn flash_guid() -> Guid {
    guid!("0A0C7E46-3399-4021-90C8-FA6D389C4BA2")
}

#[derive(Serialize, Deserialize)]
pub struct UnknownParameter {
    #[serde(skip)]
    guid: Guid,
    custom_defined: [u8; 32],
}
// https://learn.microsoft.com/en-us/windows/win32/fileio/exfat-specification#334-flash-parameters
#[derive(Serialize, Deserialize)]
pub struct FlashParameter {
    // / ## Description
    // / The ParametersGuid field shall conform to the definition provided in the
    // / Generic Parameters template (see Section 3.3.2.1). ## Value
    // / The valid value for this field, in GUID notation, is
    // / {0A0C7E46-3399-4021-90C8-FA6D389C4BA2}.
    #[serde(skip)]
    #[serde(default = "flash_guid")]
    _parameters_guid: Guid,
    ///The EraseBlockSize field shall describe the size, in bytes, of the flash
    /// media's erase block.
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

impl UnknownParameter {
    pub const NULL: UnknownParameter = UnknownParameter::new(Guid::ZERO, [0u8; 32]);
    pub const fn new(guid: Guid, custom_defined: [u8; 32]) -> Self {
        UnknownParameter {
            guid,
            custom_defined,
        }
    }

    pub fn guid(&self) -> Guid {
        self.guid
    }

    pub fn custom_defined(&self) -> [u8; 32] {
        self.custom_defined
    }
}

enum Parameter {
    Null,
    Flash(FlashParameter),
    Unknown(UnknownParameter),
}

impl Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Parameter::Null => {
                let mut seq = serializer.serialize_tuple(2)?;
                seq.serialize_element(&0u128)?;
                seq.serialize_element(&UnknownParameter::NULL)?;
                seq.end()
            }
            Parameter::Flash(flash_parameter) => {
                let mut seq = serializer.serialize_tuple(2)?;
                seq.serialize_element(&u128::from_le_bytes(flash_guid().to_bytes()))?;
                seq.serialize_element(flash_parameter)?;
                seq.end()
            }
            Parameter::Unknown(unk) => {
                let mut seq = serializer.serialize_tuple(2)?;
                seq.serialize_element(&u128::from_le_bytes(unk.guid.to_bytes()))?;
                seq.serialize_element(unk)?;
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Parameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ParameterVisitor;
        impl<'de> Visitor<'de> for ParameterVisitor {
            type Value = Parameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a tuple of guid, custom_defined")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let guid_bytes: [u8; 16] = seq.next_element()?.expect("A Guid");
                let guid = Guid::from_bytes(guid_bytes);
                if guid == flash_guid() {
                    let flash: FlashParameter = seq.next_element()?.expect("custom_defined");
                    Ok(Parameter::Flash(flash))
                } else if guid.is_zero() {
                    Ok(Parameter::Null)
                } else {
                    Ok(Parameter::Unknown(UnknownParameter::NULL))
                }
            }
        }
        deserializer.deserialize_tuple(2, ParameterVisitor)
    }
}

impl UnknownParameter {}
#[derive(Serialize, Deserialize)]
pub struct Oem {
    parameters: [Parameter; 10],
}
