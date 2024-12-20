// 3.3 - https://learn.microsoft.com/en-us/windows/win32/fileio/exfat-specification#33-main-and-backup-oem-parameters-sub-regions

mod flash;

use flash::{flash_guid, FlashParameter};
use serde::{de::Visitor, ser::SerializeTuple, Deserialize, Serialize};
use uguid::Guid;
#[derive(Serialize, Deserialize)]
pub struct UnknownParameter {
    #[serde(skip)]
    guid: Guid,
    custom_defined: [u8; 32],
}

impl UnknownParameter {
    pub const NULL: UnknownParameter =
        UnknownParameter::new(Guid::ZERO, [0u8; 32]);
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

impl Parameter {
    const NULL: Parameter = Parameter::Null;
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
                seq.serialize_element(&u128::from_le_bytes(
                    flash_guid().to_bytes(),
                ))?;
                seq.serialize_element(flash_parameter)?;
                seq.end()
            }
            Parameter::Unknown(unk) => {
                let mut seq = serializer.serialize_tuple(2)?;
                seq.serialize_element(&u128::from_le_bytes(
                    unk.guid.to_bytes(),
                ))?;
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

            fn expecting(
                &self,
                formatter: &mut core::fmt::Formatter,
            ) -> core::fmt::Result {
                formatter.write_str("a tuple of guid, custom_defined")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let guid_bytes: [u8; 16] = seq.next_element()?.expect("A Guid");
                let guid = Guid::from_bytes(guid_bytes);
                if guid == flash_guid() {
                    let flash: FlashParameter =
                        seq.next_element()?.expect("custom_defined");
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

impl Oem {
    pub fn new() -> Self {
        Self {
            parameters: [Parameter::NULL; 10],
        }
    }
    pub fn insert<Param>(&mut self, param: Param, index: usize)
    where
        Param: Into<Parameter>,
    {
        self.parameters[index] = param.into();
    }
}

#[cfg(test)]
mod tests {
    use super::{flash::FlashOptions, FlashParameter};

    #[test]
    pub fn test() {
        let param = FlashParameter::new(FlashOptions::default());
    }
}
