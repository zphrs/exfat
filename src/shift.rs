//! # Purpose
//! Used to define the power of two unshifted types: [`SectorsPerCluster`] and
//! [`BytesPerSector`] which are only used in memory; and their partners:
//! [`ShiftedSectors`] and [`ShiftedBytes`] which are actually persisted to
//! disk.

use core::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Shifted(u8);
#[derive(Clone, Copy)]
pub struct Unshifted(usize);

impl Shifted {
    pub const fn unshift(&self) -> Unshifted {
        Unshifted(2usize.pow(self.0 as u32))
    }
}

impl Unshifted {
    pub const fn shift(&self) -> Shifted {
        assert!(self.0.is_power_of_two());
        let shifted = self.0.ilog2();
        // make sure shifted is less than 256
        assert!(shifted < 256);
        Shifted(self.0.ilog2() as u8)
    }
}

impl From<Shifted> for Unshifted {
    fn from(value: Shifted) -> Self {
        value.unshift()
    }
}
impl From<Unshifted> for Shifted {
    fn from(value: Unshifted) -> Self {
        value.shift()
    }
}

impl ShiftedBytes {
    pub const fn unshift(&self) -> BytesPerSector {
        BytesPerSector(self.0.unshift())
    }

    pub const fn new(shifted_bytes: u8) -> Self {
        Self(Shifted(shifted_bytes))
    }

    pub fn inner(&self) -> u8 {
        self.0 .0
    }
}

impl ShiftedSectors {
    pub fn unshift(&self) -> SectorsPerCluster {
        SectorsPerCluster(self.0.unshift())
    }

    pub fn inner(&self) -> u8 {
        self.0 .0
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct ShiftedBytes(Shifted);
#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct ShiftedSectors(Shifted);
#[derive(Clone, Copy)]
pub struct SectorsPerCluster(Unshifted);
/// An unshifted version of [`ShiftedBytes`].
/// Stores the number of bytes in one sector.
#[derive(Clone, Copy)]
pub struct BytesPerSector(Unshifted);
impl BytesPerSector {
    pub fn shift(&self) -> ShiftedBytes {
        ShiftedBytes(self.0.shift())
    }

    pub const fn new(bytes: usize) -> Self {
        Self(Unshifted(bytes))
    }
}

impl SectorsPerCluster {
    pub fn shift(&self) -> ShiftedSectors {
        ShiftedSectors(self.0.shift())
    }
}

impl From<ShiftedBytes> for BytesPerSector {
    fn from(value: ShiftedBytes) -> Self {
        value.unshift()
    }
}
impl From<ShiftedSectors> for SectorsPerCluster {
    fn from(value: ShiftedSectors) -> Self {
        value.unshift()
    }
}

impl From<BytesPerSector> for ShiftedBytes {
    fn from(value: BytesPerSector) -> Self {
        value.shift()
    }
}

impl From<u8> for ShiftedBytes {
    fn from(value: u8) -> Self {
        Self(Shifted(value))
    }
}

impl From<u8> for ShiftedSectors {
    fn from(value: u8) -> Self {
        Self(Shifted(value))
    }
}

impl Deref for BytesPerSector {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0 .0
    }
}

impl Deref for SectorsPerCluster {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0 .0
    }
}

impl From<ShiftedBytes> for u64 {
    fn from(value: ShiftedBytes) -> Self {
        value.inner() as u64
    }
}

impl From<ShiftedSectors> for u64 {
    fn from(value: ShiftedSectors) -> Self {
        value.inner() as u64
    }
}

impl From<ShiftedBytes> for u8 {
    fn from(value: ShiftedBytes) -> Self {
        value.inner()
    }
}

impl From<ShiftedSectors> for u8 {
    fn from(value: ShiftedSectors) -> Self {
        value.inner()
    }
}

impl PartialEq<u8> for ShiftedBytes {
    fn eq(&self, other: &u8) -> bool {
        self.inner().eq(other)
    }
}

impl PartialOrd<u8> for ShiftedBytes {
    fn partial_cmp(&self, other: &u8) -> Option<core::cmp::Ordering> {
        self.inner().partial_cmp(other)
    }
}

impl PartialEq<u8> for ShiftedSectors {
    fn eq(&self, other: &u8) -> bool {
        self.inner().eq(other)
    }
}

impl PartialOrd<u8> for ShiftedSectors {
    fn partial_cmp(&self, other: &u8) -> Option<core::cmp::Ordering> {
        self.inner().partial_cmp(other)
    }
}
