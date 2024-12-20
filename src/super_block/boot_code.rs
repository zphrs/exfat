use macros::DiskLayout;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct BootCode(
    /// ## Description
    /// The BootCode field shall contain boot-strapping instructions.
    /// ## Value
    /// Implementations may populate this field with the CPU instructions
    /// necessary for boot-strapping a computer system. Implementations which
    /// don't provide boot-strapping instructions shall initialize each byte in
    /// this field to F4h (the halt instruction for CPUs common in personal
    /// computers) as part of their format operation.
    #[serde(with = "BigArray")]
    [u8; 390],
    /// ## Description
    /// The BootSignature field shall describe whether the intent of a given
    /// sector is for it to be a Boot Sector or not.
    /// ## Value
    /// The valid value for this field is AA55h. Any other value in this field
    /// invalidates its respective Boot Sector. Implementations should verify
    /// the contents of this field prior to depending on any other field in its
    /// respective Boot Sector.
    u16,
);

impl BootCode {
    pub fn new(boot_code: &[u8]) -> Self {
        // check length is not longer than 390
        if boot_code.len() > 390 {
            panic!("Boot code is too long!");
        }
        let mut out = Self::default();
        out.0[0..boot_code.len()].copy_from_slice(boot_code);
        out
    }
}

impl Default for BootCode {
    fn default() -> Self {
        Self([0xf4; 390], 0xAA55)
    }
}
