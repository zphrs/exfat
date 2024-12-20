use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct VolumeFlags {
    flags: u16,
}

impl VolumeFlags {
    /// The ActiveFat field shall describe which FAT and Allocation Bitmap are
    /// active (and implementations shall use), as follows:
    ///
    /// - 0, which means the First FAT and First Allocation Bitmap are active
    ///
    /// - 1, which means the Second FAT and Second Allocation Bitmap are active
    ///   and is possible only when the NumberOfFats field contains the value 2
    ///
    /// Implementations shall consider the inactive FAT and Allocation Bitmap as
    /// stale. Only TexFAT-aware implementations shall switch the active FAT and
    /// Allocation Bitmaps (see Section 7.1).
    const ACTIVE_FAT: u16 = 0b10000000;
    /// The VolumeDirty field shall describe whether the volume is dirty or not,
    /// as follows:
    ///
    /// - 0, which means the volume is probably in a consistent state
    ///
    /// - 1, which means the volume is probably in an inconsistent state
    ///
    /// Implementations should set the value of this field to 1 upon
    /// encountering file system metadata inconsistencies which they do not
    /// resolve. If, upon mounting a volume, the value of this field is 1, only
    /// implementations which resolve file system metadata inconsistencies may
    /// clear the value of this field to 0. Such implementations shall only
    /// clear the value of this field to 0 after ensuring the file system is in
    /// a consistent state.
    ///
    /// If, upon mounting a volume, the value of this field is 0,
    /// implementations should set this field to 1 before updating file system
    /// metadata and clear this field to 0 afterwards, similar to the
    /// recommended write ordering described in Section 8.1.
    const VOLUME_DIRTY: u16 = 0b01000000;
    /// The MediaFailure field shall describe whether an implementation has
    /// discovered media failures or not, as follows:

    /// - 0, which means the hosting media has not reported failures or any
    ///   known failures are already recorded in the FAT as "bad" clusters
    ///
    /// - 1, which means the hosting media has reported failures (i.e. has
    ///   failed read or write operations)
    ///
    /// An implementation should set this field to 1 when:
    ///
    /// 1. The hosting media fails access attempts to any region in the volume
    ///
    /// 2. The implementation has exhausted access retry algorithms, if any

    /// If, upon mounting a volume, the value of this field is 1,
    /// implementations which scan the entire volume for media failures and
    /// record all failures as "bad" clusters in the FAT (or otherwise resolve
    /// media failures) may clear the value of this field to 0.
    const MEDIA_FAILURE: u16 = 0b00100000;
    /// The ClearToZero field does not have significant meaning in this
    /// specification.
    ///
    /// The valid values for this field are:
    ///
    /// - 0, which does not have any particular meaning
    ///
    /// - 1, which means implementations shall clear this field to 0 prior to
    ///   modifying any file system structures, directories, or files
    pub const CLEAR_TO_ZERO: u16 = 0b00010000;

    fn set_flag(&mut self, flag: u16, on: bool) {
        if on {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    pub fn set_active_fat(&mut self, second_active: bool) {
        self.set_flag(Self::ACTIVE_FAT, second_active);
    }
    pub fn set_volume_dirty(&mut self, inconsistent: bool) {
        self.set_flag(Self::VOLUME_DIRTY, inconsistent);
    }
    pub fn set_media_failure(&mut self, failed: bool) {
        self.set_flag(Self::MEDIA_FAILURE, failed);
    }
    pub fn set_clear_to_zero(&mut self, clear_before_modification: bool) {
        self.set_flag(Self::CLEAR_TO_ZERO, clear_before_modification);
    }
}

impl Default for VolumeFlags {
    fn default() -> Self {
        Self {
            flags: Default::default(),
        }
    }
}
