pub mod boot_code;
pub mod extended_boot_code;
mod extended_boot_sector;
mod must_be_zero;
mod volume_flags;

use std::{cmp::min, ops::Deref};

use crate::shift::{BytesPerSector, SectorsPerCluster, ShiftedBytes, ShiftedSectors};

use self::{boot_code::BootCode, must_be_zero::MustBeZero, volume_flags::VolumeFlags};
use macros::DiskLayout;
use rand::{rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};

// use macros::DiskLayout;
#[derive(DiskLayout, Default, Serialize, Deserialize, Clone)]
pub struct BootSector {
    /// ## Description
    /// The JumpBoot field shall contain the jump instruction for CPUs common in
    /// personal computers, which, when executed, "jumps" the CPU to execute the
    /// boot-strapping instructions in the BootCode field.
    /// ## Value
    /// The valid value for this field is (in order of low-order byte to
    /// high-order byte) EBh 76h 90h.
    jump_boot: [u8; 3],
    /// ## Description
    /// The FileSystemName field shall contain the name of the file system on
    /// the volume.
    /// ## Value
    /// The valid value for this field is, in ASCII characters, "EXFAT   ",
    /// which includes three trailing white spaces.
    file_system_name: [u8; 8],
    must_be_zero: MustBeZero,
    /// ## Description
    /// The PartitionOffset field shall describe the media-relative sector
    /// offset of the partition which hosts the given exFAT volume. This field
    /// aids boot-strapping from the volume using extended INT 13h on personal
    /// computers.
    /// ## Value
    /// All possible values for this field are valid; however, the value 0
    /// indicates implementations shall ignore this field.
    partition_offset: u64,
    /// ## Description
    /// The VolumeLength field shall describe the size of the given exFAT volume
    /// in sectors.
    /// ## Value
    /// The valid range of values for this field shall be:

    /// - At least `(2**20)/(2**BytesPerSectorShift)`, which ensures the
    ///   smallest volume is no less than 1MB

    /// - At most 2**64- 1, the largest value this field can describe.
    ///
    ///   However, if the size of the Excess Space sub-region is 0, then the
    ///   largest value of this field is
    /// `ClusterHeapOffset + (232 - 11) * 2**SectorsPerClusterShift`.
    #[min(2u64.pow(20) / (2u64.pow(self.bytes_per_sector_shift)))]
    #[max(u64::MAX)]
    volume_length: u64,
    /// ## Description
    /// The FatOffset field shall describe the volume-relative sector offset of
    /// the First FAT. This field enables implementations to align the First FAT
    /// to the characteristics of the underlying storage media.
    /// ## Value
    /// The valid range of values for this field shall be:
    /// - At least 24, which accounts for the sectors the Main Boot and Backup
    /// Boot regions consume

    /// - At most `ClusterHeapOffset - (FatLength * NumberOfFats)`, which
    ///   accounts
    /// for the sectors the Cluster Heap consumes
    #[min(24)]
    #[max(self.cluster_heap_offset - self.fat_length * self.number_of_fats)]
    fat_offset: u32,
    /// ## Description
    /// The FatLength field shall describe the length, in sectors, of each FAT
    /// table (the volume may contain up to two FATs).
    /// ## Value
    /// The valid range of values for this field shall be:
    ///
    /// - At least `(ClusterCount + 2) * 2**2/ 2**BytesPerSectorShift` rounded
    ///   up to the nearest integer, which ensures each FAT has sufficient space
    ///   for describing all the clusters in the Cluster Heap
    ///
    /// - At most `(ClusterHeapOffset - FatOffset) / NumberOfFats` rounded down
    ///   to the nearest integer, which ensures the FATs exist before the
    ///   Cluster Heap
    #[min((self.cluster_count + 2) * 4 / 2u32.pow(self.bytes_per_sector_shift))]
    #[max((self.cluster_heap_offset - self.fat_offset) / self.number_of_fats)]
    fat_length: u32,
    /// ## Description
    /// The ClusterHeapOffset field shall describe the volume-relative sector
    /// offset of the Cluster Heap. This field enables implementations to align
    /// the Cluster Heap to the characteristics of the underlying storage media.
    /// ## Value
    /// The valid range of values for this field shall be:
    ///
    /// - At least FatOffset + FatLength * NumberOfFats, to account for the
    ///   sectors all the preceding regions consume
    ///
    /// - At most `2**32 - 1` or `VolumeLength - (ClusterCount *
    ///   2**SectorsPerClusterShift)`, whichever calculation is less
    #[min(self.fat_offset + self.fat_length * self.number_of_fats)]
    #[max(cmp::min(u32::MAX - 1, self.volume_length - (self.cluster_count * 2u32.pow(self.sectors_per_cluster_shift))))]
    cluster_heap_offset: u32,
    /// ## Description
    /// The ClusterCount field shall describe the number of clusters the Cluster
    /// Heap contains.
    /// ## Value
    /// The valid value for this field shall be the lesser of the following:
    ///
    /// - `(VolumeLength - ClusterHeapOffset) / 2**SectorsPerClusterShift`
    ///   rounded down to the nearest integer, which is exactly the number of
    ///   clusters which can fit between the beginning of the Cluster Heap and
    ///   the end of the volume
    ///
    /// - `2**32- 11`, which is the maximum number of clusters a FAT can
    ///   describe
    #[min(self.volume_length - self.cluster_heap_offset / 2u32.pow(self.sectors_per_cluster_shift))]
    #[max(u32::MAX - 11)]
    cluster_count: u32,
    /// ## Description
    /// The FirstClusterOfRootDirectory field shall contain the cluster index of
    /// the first cluster of the root directory. Implementations should make
    /// every effort to place the first cluster of the root directory in the
    /// first non-bad cluster after the clusters the Allocation Bitmap and
    /// Up-case Table consume.
    /// ## Value
    /// The valid range of values for this field shall be:
    ///
    /// - At least 2, the index of the first cluster in the Cluster Heap
    ///
    /// - At most ClusterCount + 1, the index of the last cluster in the Cluster
    ///   Heap
    #[min(2)]
    #[max(self.cluster_count + 1)]
    first_cluster_of_root_directory: u32,
    /// ## Description
    /// The VolumeSerialNumber field shall contain a unique serial number. This
    /// assists implementations to distinguish among different exFAT volumes.
    /// Implementations should generate the serial number by combining the date
    /// and time of formatting the exFAT volume. The mechanism for combining
    /// date and time to form a serial number is implementation-specific.
    /// ## Value
    /// All possible values for this field are valid.
    volume_serial_number: u32,
    /// ## Description
    /// The FileSystemRevision field shall describe the major and minor revision
    /// numbers of the exFAT structures on the given volume.
    ///
    /// The high-order byte is the major revision number and the low-order byte
    /// is the minor revision number. For example, if the high-order byte
    /// contains the value 01h and if the low-order byte contains the value 05h,
    /// then the FileSystemRevision field describes the revision number 1.05.
    /// Likewise, if the high-order byte contains the value 0Ah and if the
    /// low-order byte contains the value 0Fh, then the FileSystemRevision field
    /// describes the revision number 10.15.
    /// ## Values
    /// - At least 0 for the low-order byte and 1 for the high-order byte

    /// - At most 99 for the low-order byte and 99 for the high-order byte
    file_system_revision: [u8; 2],
    /// The VolumeFlags field shall contain flags which indicate the status of
    /// various file system structures on the exFAT volume (see VolumeFlags).
    ///
    /// Implementations shall not include this field when computing its
    /// respective Main Boot or Backup Boot region checksum. When referring to
    /// the Backup Boot Sector, implementations shall treat this field as stale.
    volume_flags: VolumeFlags,
    /// The BytesPerSectorShift field shall describe the bytes per sector
    /// expressed as log2(N), where N is the number of bytes per sector. For
    /// example, for 512 bytes per sector, the value of this field is 9.
    ///
    /// The valid range of values for this field shall be:
    /// - At least 9 (sector size of 512 bytes), which is the smallest sector
    ///   possible for an exFAT volume
    /// - At most 12 (sector size of 4096 bytes), which is the memory page size
    ///   of CPUs common in personal computers
    #[min(9)]
    #[max(12)]
    bytes_per_sector_shift: ShiftedBytes,
    /// The SectorsPerClusterShift field shall describe the sectors per cluster
    /// expressed as log2(N), where N is number of sectors per cluster. For
    /// example, for 8 sectors per cluster, the value of this field is 3.
    ///
    /// The valid range of values for this field shall be:
    /// - At least 0 (1 sector per cluster), which is the smallest cluster
    ///   possible
    /// - At most 25 - BytesPerSectorShift, which evaluates to a cluster size of
    ///   32MB
    #[min(0)]
    #[max(25 - self.bytes_per_sector_shift)]
    sectors_per_cluster_shift: ShiftedSectors,
    /// The NumberOfFats field shall describe the number of FATs and Allocation
    /// Bitmaps the volume contains.
    ///
    /// The valid range of values for this field shall be:
    ///
    /// - 1, which indicates the volume only contains the First FAT and First
    ///   Allocation Bitmap
    ///
    /// - 2, which indicates the volume contains the First FAT, Second FAT,
    ///   First Allocation Bitmap, and Second Allocation Bitmap; this value is
    ///   only valid for TexFAT volumes
    number_of_fats: u8,
    /// The DriveSelect field shall contain the extended INT 13h drive number,
    /// which aids boot-strapping from this volume using extended INT 13h on
    /// personal computers.
    ///
    /// All possible values for this field are valid. Similar fields in previous
    /// FAT-based file systems frequently contained the value 80h.
    drive_select: u8,
    /// The PercentInUse field shall describe the percentage of clusters in the
    /// Cluster Heap which are allocated.
    ///
    /// The valid range of values for this field shall be:
    ///
    /// - Between 0 and 100 inclusively, which is the percentage of allocated
    ///   clusters in the Cluster Heap, rounded down to the nearest integer
    ///
    /// - Exactly FFh, which indicates the percentage of allocated clusters in
    ///   the Cluster Heap is not available
    ///
    /// Implementations shall change the value of this field to reflect changes
    /// in the allocation of clusters in the Cluster Heap or shall change it to
    /// FFh.
    ///
    /// Implementations shall not include this field when computing its
    /// respective Main Boot or Backup Boot region checksum. When referring to
    /// the Backup Boot Sector, implementations shall treat this field as stale.
    percent_in_use: u8,
    reserved: [u8; 7],
    /// ## Description
    /// The BootCode field shall contain boot-strapping instructions.
    /// ## Value
    /// Implementations may populate this field with the CPU instructions
    /// necessary for boot-strapping a computer system. Implementations which
    /// don't provide boot-strapping instructions shall initialize each byte in
    /// this field to F4h (the halt instruction for CPUs common in personal
    /// computers) as part of their format operation.
    boot_code: BootCode,
    #[size(2u64.pow(self.bytes_per_sector_shift) - 512)]
    excess_space: (),
}

impl BootSector {
    pub fn new(
        bytes_per_sector_shift: u8,
        sectors_per_cluster_shift: u8,
        cluster_heap_offset: u32,
        boot_code: BootCode,
    ) -> Self {
        let mut out = Self::default();
        out.jump_boot = [0xEB, 0x76, 0x90];
        out.file_system_name = *b"EXFAT   ";
        out.partition_offset = 0;

        out.boot_code = boot_code;

        out.bytes_per_sector_shift = (bytes_per_sector_shift).into();
        out.sectors_per_cluster_shift = sectors_per_cluster_shift.into();
        out.cluster_heap_offset = cluster_heap_offset;

        out.volume_length = out.cluster_heap_offset as u64
            + (2u64.pow(32u32) - 11) * (*out.sectors_per_cluster_shift.unshift() as u64);
        out.fat_offset = 24;
        out.cluster_count = min(
            out.volume_length as i64
                - out.cluster_heap_offset as i64
                    / (*out.sectors_per_cluster_shift.unshift() as i64),
            (u32::MAX - 11) as i64,
        )
        .try_into()
        .unwrap();

        out.fat_length = ((out.cluster_count as u64 + 2) * 4
            / (*out.bytes_per_sector_shift.unshift() as u64))
            .try_into()
            .unwrap();
        out.first_cluster_of_root_directory = 2;
        out.volume_serial_number = OsRng.gen(); // HACK should instead use current date and time.
        out.file_system_revision = [0, 1];
        out.number_of_fats = 2;
        out.drive_select = 0x80;
        out.percent_in_use = 0;
        // out.boot_code = BootCode::default();
        out
    }

    pub fn bytes_per_sector(&self) -> BytesPerSector {
        self.bytes_per_sector_shift.unshift()
    }
    pub fn sectors_per_cluster(&self) -> SectorsPerCluster {
        self.sectors_per_cluster_shift.unshift()
    }
    pub(crate) fn set_boot_code(&mut self, boot_code: BootCode) {
        self.boot_code = boot_code;
    }
    // FIXME should be a checksum over the whole main boot region
    pub fn checksum(&self) -> u32 {
        let serialized = bincode::serialize(self).unwrap();
        let mut checksum = 0u32;
        for (i, byte) in serialized.iter().enumerate() {
            if [
                106, 107, // volumeFlags
                112, // percentInUse
            ]
            .contains(&i)
            {
                continue;
            }
            checksum = checksum.rotate_right(1) + (*byte as u32);
        }
        checksum
    }
}
#[cfg(test)]
mod tests {
    use bincode::Options;

    use super::{boot_code::BootCode, BootSector};

    #[test]
    pub fn serialize() {
        let bs = BootSector::new(9, 5, 12000, BootCode::default());
        let my_options = bincode::DefaultOptions::new()
            .allow_trailing_bytes()
            .with_fixint_encoding()
            .with_little_endian();
        let serialized = my_options.serialize(&bs).unwrap();
        println!("{serialized:?}");
    }
}
