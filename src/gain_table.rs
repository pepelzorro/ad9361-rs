//! Gain table configuration

use crate::bindings;
use getset::{CopyGetters, Setters};

/// The AD9361 supports both full and split gain tables
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GainTableKind {
    Full,
    Split,
}

/// Gain table
///
/// The [Default](#impl-Default) value of this type matches the values from the
/// [example
/// project](https://github.com/analogdevicesinc/no-OS/tree/master/projects/ad9361/src)
/// in the [no-OS](https://github.com/analogdevicesinc/no-OS) library.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GainTable {
    // need to use 6 so that split gain table is at the right index for
    // midfrequency lna_table and mixer_table lookups
    info: [bindings::gain_table_info; 6],
    // index of the active gain info table
    index: usize,
    table: [[u8; 3]; 90],
    abs_gain_tbl: [i8; 90],
}
impl GainTable {
    /// Set internal self-referential pointers and return a pointer to the
    /// array of gain_table_info structs
    ///
    /// # Safety
    ///
    /// `self` must not be moved after calling this method
    pub(crate) unsafe fn set_ptr(&mut self) -> *mut bindings::gain_table_info {
        // set self-referential fields in info
        for i in 0..=self.index {
            self.info[i].tab = &mut self.table[0];
            self.info[i].abs_gain_tbl = &mut self.abs_gain_tbl[0];
        }
        // return pointer to array of gain info structs
        &mut self.info[0]
    }
}

/// Represents an entry in a gain table
#[derive(Clone, Copy, Debug, CopyGetters, Setters)]
#[get_copy = "pub"]
#[set = "pub"]
pub struct GainEntry {
    reg131: u8,
    reg132: u8,
    reg133: u8,
    abs_gain: i8,
}

/// Methods for mutating the gain table set
impl GainTable {
    /// Returns the entry at index from a gain table
    ///
    /// index in the range [1, 90]
    pub fn get_entry(&self, index: usize) -> GainEntry {
        debug_assert!(index > 0);
        debug_assert!(index <= self.info[0].max_index.into());

        GainEntry {
            reg131: self.table[index - 1][0],
            reg132: self.table[index - 1][1],
            reg133: self.table[index - 1][2],
            abs_gain: self.abs_gain_tbl[index - 1],
        }
    }
    /// Sets the entry at index in a given gain table. If not already the case,
    /// expands the table to at least `index` entries.
    ///
    /// index must be less than 90
    pub fn set_entry(&mut self, index: usize, e: GainEntry) {
        debug_assert!(index > 0);
        debug_assert!(index <= 90);

        self.table[index - 1][0] = e.reg131;
        self.table[index - 1][1] = e.reg132;
        self.table[index - 1][2] = e.reg133;
        self.abs_gain_tbl[index - 1] = e.abs_gain;
        self.info[self.index].max_index =
            core::cmp::max(index as u8, self.info[self.index].max_index);
    }
    /// Gain table kind
    pub fn kind(&self) -> GainTableKind {
        match self.info[self.index].split_table {
            1 => GainTableKind::Split,
            _ => GainTableKind::Full,
        }
    }
    /// Maximum index currently used in this gain table
    pub fn max_index(&self) -> usize {
        self.info[self.index].max_index.into()
    }
}

impl GainTable {
    /// New gain table, with default values from the [example
    /// project](https://github.com/analogdevicesinc/no-OS/tree/master/projects/ad9361/src)
    /// in the [no-OS](https://github.com/analogdevicesinc/no-OS) library.
    pub const fn new_from_recommended(
        kind: GainTableKind,
        frequency: u64,
    ) -> Self {
        let index = if frequency < 1_300_000_000 {
            0
        } else if frequency < 4_000_000_000 {
            1
        } else {
            2
        };
        let gt_null = bindings::gain_table_info {
            start: 0,
            end: 0,
            max_index: 0,
            split_table: 0,
            abs_gain_tbl: core::ptr::null_mut(),
            tab: core::ptr::null_mut(),
        };
        if matches!(kind, GainTableKind::Full) {
            let info = [
                bindings::gain_table_info {
                    start: 0,
                    end: 6_000_000_000,
                    max_index: 77, // SIZE_FULL_TABLE
                    split_table: 0,
                    abs_gain_tbl: core::ptr::null_mut(),
                    tab: core::ptr::null_mut(),
                },
                gt_null,
                gt_null,
                gt_null,
                gt_null,
                gt_null,
            ];

            #[rustfmt::skip]
            let table = [
            [  /* 800 MHz */
                [0x00, 0x00, 0x20], [0x00, 0x00, 0x00], [0x00, 0x00, 0x00],
                [0x00, 0x01, 0x00], [0x00, 0x02, 0x00], [0x00, 0x03, 0x00],
                [0x00, 0x04, 0x00], [0x00, 0x05, 0x00], [0x01, 0x03, 0x20],
                [0x01, 0x04, 0x00], [0x01, 0x05, 0x00], [0x01, 0x06, 0x00],
                [0x01, 0x07, 0x00], [0x01, 0x08, 0x00], [0x01, 0x09, 0x00],
                [0x01, 0x0A, 0x00], [0x01, 0x0B, 0x00], [0x01, 0x0C, 0x00],
                [0x01, 0x0D, 0x00], [0x01, 0x0E, 0x00], [0x02, 0x09, 0x20],
                [0x02, 0x0A, 0x00], [0x02, 0x0B, 0x00], [0x02, 0x0C, 0x00],
                [0x02, 0x0D, 0x00], [0x02, 0x0E, 0x00], [0x02, 0x0F, 0x00],
                [0x02, 0x10, 0x00], [0x02, 0x2B, 0x20], [0x02, 0x2C, 0x00],
                [0x04, 0x28, 0x20], [0x04, 0x29, 0x00], [0x04, 0x2A, 0x00],
                [0x04, 0x2B, 0x00], [0x24, 0x20, 0x20], [0x24, 0x21, 0x00],
                [0x44, 0x20, 0x20], [0x44, 0x21, 0x00], [0x44, 0x22, 0x00],
                [0x44, 0x23, 0x00], [0x44, 0x24, 0x00], [0x44, 0x25, 0x00],
                [0x44, 0x26, 0x00], [0x44, 0x27, 0x00], [0x44, 0x28, 0x00],
                [0x44, 0x29, 0x00], [0x44, 0x2A, 0x00], [0x44, 0x2B, 0x00],
                [0x44, 0x2C, 0x00], [0x44, 0x2D, 0x00], [0x44, 0x2E, 0x00],
                [0x44, 0x2F, 0x00], [0x44, 0x30, 0x00], [0x44, 0x31, 0x00],
                [0x44, 0x32, 0x00], [0x64, 0x2E, 0x20], [0x64, 0x2F, 0x00],
                [0x64, 0x30, 0x00], [0x64, 0x31, 0x00], [0x64, 0x32, 0x00],
                [0x64, 0x33, 0x00], [0x64, 0x34, 0x00], [0x64, 0x35, 0x00],
                [0x64, 0x36, 0x00], [0x64, 0x37, 0x00], [0x64, 0x38, 0x00],
                [0x65, 0x38, 0x20], [0x66, 0x38, 0x20], [0x67, 0x38, 0x20],
                [0x68, 0x38, 0x20], [0x69, 0x38, 0x20], [0x6A, 0x38, 0x20],
                [0x6B, 0x38, 0x20], [0x6C, 0x38, 0x20], [0x6D, 0x38, 0x20],
                [0x6E, 0x38, 0x20], [0x6F, 0x38, 0x20],

                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],

            ],[  /* 2300 MHz */
                [0x00, 0x00, 0x20], [0x00, 0x00, 0x00], [0x00, 0x00, 0x00],
                [0x00, 0x01, 0x00], [0x00, 0x02, 0x00], [0x00, 0x03, 0x00],
                [0x00, 0x04, 0x00], [0x00, 0x05, 0x00], [0x01, 0x03, 0x20],
                [0x01, 0x04, 0x00], [0x01, 0x05, 0x00], [0x01, 0x06, 0x00],
                [0x01, 0x07, 0x00], [0x01, 0x08, 0x00], [0x01, 0x09, 0x00],
                [0x01, 0x0A, 0x00], [0x01, 0x0B, 0x00], [0x01, 0x0C, 0x00],
                [0x01, 0x0D, 0x00], [0x01, 0x0E, 0x00], [0x02, 0x09, 0x20],
                [0x02, 0x0A, 0x00], [0x02, 0x0B, 0x00], [0x02, 0x0C, 0x00],
                [0x02, 0x0D, 0x00], [0x02, 0x0E, 0x00], [0x02, 0x0F, 0x00],
                [0x02, 0x10, 0x00], [0x02, 0x2B, 0x20], [0x02, 0x2C, 0x00],
                [0x04, 0x27, 0x20], [0x04, 0x28, 0x00], [0x04, 0x29, 0x00],
                [0x04, 0x2A, 0x00], [0x04, 0x2B, 0x00], [0x24, 0x21, 0x20],
                [0x24, 0x22, 0x00], [0x44, 0x20, 0x20], [0x44, 0x21, 0x00],
                [0x44, 0x22, 0x00], [0x44, 0x23, 0x00], [0x44, 0x24, 0x00],
                [0x44, 0x25, 0x00], [0x44, 0x26, 0x00], [0x44, 0x27, 0x00],
                [0x44, 0x28, 0x00], [0x44, 0x29, 0x00], [0x44, 0x2A, 0x00],
                [0x44, 0x2B, 0x00], [0x44, 0x2C, 0x00], [0x44, 0x2D, 0x00],
                [0x44, 0x2E, 0x00], [0x44, 0x2F, 0x00], [0x44, 0x30, 0x00],
                [0x44, 0x31, 0x00], [0x64, 0x2E, 0x20], [0x64, 0x2F, 0x00],
                [0x64, 0x30, 0x00], [0x64, 0x31, 0x00], [0x64, 0x32, 0x00],
                [0x64, 0x33, 0x00], [0x64, 0x34, 0x00], [0x64, 0x35, 0x00],
                [0x64, 0x36, 0x00], [0x64, 0x37, 0x00], [0x64, 0x38, 0x00],
                [0x65, 0x38, 0x20], [0x66, 0x38, 0x20], [0x67, 0x38, 0x20],
                [0x68, 0x38, 0x20], [0x69, 0x38, 0x20], [0x6A, 0x38, 0x20],
                [0x6B, 0x38, 0x20], [0x6C, 0x38, 0x20], [0x6D, 0x38, 0x20],
                [0x6E, 0x38, 0x20], [0x6F, 0x38, 0x20],

                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],

            ],[  /* 5500 MHz */
                [0x00, 0x00, 0x20], [0x00, 0x00, 0x00], [0x00, 0x00, 0x00],
                [0x00, 0x00, 0x00], [0x00, 0x00, 0x00], [0x00, 0x01, 0x00],
                [0x00, 0x02, 0x00], [0x00, 0x03, 0x00], [0x01, 0x01, 0x20],
                [0x01, 0x02, 0x00], [0x01, 0x03, 0x00], [0x01, 0x04, 0x20],
                [0x01, 0x05, 0x00], [0x01, 0x06, 0x00], [0x01, 0x07, 0x00],
                [0x01, 0x08, 0x00], [0x01, 0x09, 0x00], [0x01, 0x0A, 0x00],
                [0x01, 0x0B, 0x00], [0x01, 0x0C, 0x00], [0x02, 0x08, 0x20],
                [0x02, 0x09, 0x00], [0x02, 0x0A, 0x00], [0x02, 0x0B, 0x20],
                [0x02, 0x0C, 0x00], [0x02, 0x0D, 0x00], [0x02, 0x0E, 0x00],
                [0x02, 0x0F, 0x00], [0x02, 0x2A, 0x20], [0x02, 0x2B, 0x00],
                [0x04, 0x27, 0x20], [0x04, 0x28, 0x00], [0x04, 0x29, 0x00],
                [0x04, 0x2A, 0x00], [0x04, 0x2B, 0x00], [0x04, 0x2C, 0x00],
                [0x04, 0x2D, 0x00], [0x24, 0x20, 0x20], [0x24, 0x21, 0x00],
                [0x24, 0x22, 0x00], [0x44, 0x20, 0x20], [0x44, 0x21, 0x00],
                [0x44, 0x22, 0x00], [0x44, 0x23, 0x00], [0x44, 0x24, 0x00],
                [0x44, 0x25, 0x00], [0x44, 0x26, 0x00], [0x44, 0x27, 0x00],
                [0x44, 0x28, 0x00], [0x44, 0x29, 0x00], [0x44, 0x2A, 0x00],
                [0x44, 0x2B, 0x00], [0x44, 0x2C, 0x00], [0x44, 0x2D, 0x00],
                [0x44, 0x2E, 0x00], [0x64, 0x2E, 0x20], [0x64, 0x2F, 0x00],
                [0x64, 0x30, 0x00], [0x64, 0x31, 0x00], [0x64, 0x32, 0x00],
                [0x64, 0x33, 0x00], [0x64, 0x34, 0x00], [0x64, 0x35, 0x00],
                [0x64, 0x36, 0x00], [0x64, 0x37, 0x00], [0x64, 0x38, 0x00],
                [0x65, 0x38, 0x20], [0x66, 0x38, 0x20], [0x67, 0x38, 0x20],
                [0x68, 0x38, 0x20], [0x69, 0x38, 0x20], [0x6A, 0x38, 0x20],
                [0x6B, 0x38, 0x20], [0x6C, 0x38, 0x20], [0x6D, 0x38, 0x20],
                [0x6E, 0x38, 0x20], [0x6F, 0x38, 0x20],

                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
            ]
            ];

            let abs_gain_tbl = [
                [
                    /* 800 MHz */
                    -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                    14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
                    29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                    44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                    59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                [
                    /* 2300 MHz */
                    -3, -3, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
                    12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
                    57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                [
                    /* 5500 MHz */
                    -10, -10, -10, -10, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1,
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                    47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            ];

            GainTable {
                info,
                index: 0, // active table at index 0
                abs_gain_tbl: abs_gain_tbl[index],
                table: table[index],
            }
        } else {
            let info = [
                gt_null,
                gt_null,
                gt_null,
                gt_null,
                bindings::gain_table_info {
                    start: 0,
                    end: 6_000_000_000,
                    max_index: 41, // SIZE_SPLIT_TABLE
                    split_table: 1,
                    abs_gain_tbl: core::ptr::null_mut(),
                    tab: core::ptr::null_mut(),
                },
                gt_null,
            ];

            #[rustfmt::skip]
            let table = [
            [  /* 800 MHz */
                [0x00, 0x18, 0x20], [0x00, 0x18, 0x00], [0x00, 0x18, 0x00],
                [0x00, 0x18, 0x00], [0x00, 0x18, 0x00], [0x00, 0x18, 0x00],
                [0x00, 0x18, 0x20], [0x01, 0x18, 0x20], [0x02, 0x18, 0x20],
                [0x04, 0x18, 0x20], [0x04, 0x38, 0x20], [0x05, 0x38, 0x20],
                [0x06, 0x38, 0x20], [0x07, 0x38, 0x20], [0x08, 0x38, 0x20],
                [0x09, 0x38, 0x20], [0x0A, 0x38, 0x20], [0x0B, 0x38, 0x20],
                [0x0C, 0x38, 0x20], [0x0D, 0x38, 0x20], [0x0E, 0x38, 0x20],
                [0x0F, 0x38, 0x20], [0x24, 0x38, 0x20], [0x25, 0x38, 0x20],
                [0x44, 0x38, 0x20], [0x45, 0x38, 0x20], [0x46, 0x38, 0x20],
                [0x47, 0x38, 0x20], [0x48, 0x38, 0x20], [0x64, 0x38, 0x20],
                [0x65, 0x38, 0x20], [0x66, 0x38, 0x20], [0x67, 0x38, 0x20],
                [0x68, 0x38, 0x20], [0x69, 0x38, 0x20], [0x6A, 0x38, 0x20],
                [0x6B, 0x38, 0x20], [0x6C, 0x38, 0x20], [0x6D, 0x38, 0x20],
                [0x6E, 0x38, 0x20], [0x6F, 0x38, 0x20],

                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],

            ],[  /* 2300 MHz */
                [0x00, 0x18, 0x20], [0x00, 0x18, 0x00], [0x00, 0x18, 0x00],
                [0x00, 0x18, 0x00], [0x00, 0x18, 0x00], [0x00, 0x18, 0x00],
                [0x00, 0x18, 0x00], [0x00, 0x18, 0x20], [0x01, 0x18, 0x20],
                [0x02, 0x18, 0x20], [0x04, 0x18, 0x20], [0x04, 0x38, 0x20],
                [0x05, 0x38, 0x20], [0x06, 0x38, 0x20], [0x07, 0x38, 0x20],
                [0x08, 0x38, 0x20], [0x09, 0x38, 0x20], [0x0A, 0x38, 0x20],
                [0x0B, 0x38, 0x20], [0x0C, 0x38, 0x20], [0x0D, 0x38, 0x20],
                [0x0E, 0x38, 0x20], [0x0F, 0x38, 0x20], [0x25, 0x38, 0x20],
                [0x26, 0x38, 0x20], [0x44, 0x38, 0x20], [0x45, 0x38, 0x20],
                [0x46, 0x38, 0x20], [0x47, 0x38, 0x20], [0x64, 0x38, 0x20],
                [0x65, 0x38, 0x20], [0x66, 0x38, 0x20], [0x67, 0x38, 0x20],
                [0x68, 0x38, 0x20], [0x69, 0x38, 0x20], [0x6A, 0x38, 0x20],
                [0x6B, 0x38, 0x20], [0x6C, 0x38, 0x20], [0x6D, 0x38, 0x20],
                [0x6E, 0x38, 0x20], [0x6F, 0x38, 0x20],

                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],

            ],[  /* 5500 MHz */
                [0x00, 0x18, 0x20], [0x00, 0x18, 0x00], [0x00, 0x18, 0x00],
                [0x00, 0x18, 0x00], [0x00, 0x18, 0x00], [0x00, 0x18, 0x00],
                [0x00, 0x18, 0x00], [0x00, 0x18, 0x00], [0x00, 0x18, 0x00],
                [0x00, 0x18, 0x00], [0x01, 0x18, 0x20], [0x02, 0x18, 0x20],
                [0x04, 0x18, 0x20], [0x04, 0x38, 0x20], [0x05, 0x38, 0x20],
                [0x06, 0x38, 0x20], [0x07, 0x38, 0x20], [0x08, 0x38, 0x20],
                [0x09, 0x38, 0x20], [0x0A, 0x38, 0x20], [0x0B, 0x38, 0x20],
                [0x0C, 0x38, 0x20], [0x0D, 0x38, 0x20], [0x0E, 0x38, 0x20],
                [0x0F, 0x38, 0x20], [0x62, 0x38, 0x20], [0x25, 0x38, 0x20],
                [0x26, 0x38, 0x20], [0x44, 0x38, 0x20], [0x64, 0x38, 0x20],
                [0x65, 0x38, 0x20], [0x66, 0x38, 0x20], [0x67, 0x38, 0x20],
                [0x68, 0x38, 0x20], [0x69, 0x38, 0x20], [0x6A, 0x38, 0x20],
                [0x6B, 0x38, 0x20], [0x6C, 0x38, 0x20], [0x6D, 0x38, 0x20],
                [0x6E, 0x38, 0x20], [0x6F, 0x38, 0x20],

                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
                [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0], [0,0,0],
            ]
            ];
            let abs_gain_tbl = [
                [
                    /* 800 MHz */
                    -1, -1, -1, -1, -1, -1, -1, 2, 8, 13, 19, 20, 21, 22, 23,
                    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                    39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0,
                ],
                [
                    /* 2300 MHz */
                    -3, -3, -3, -3, -3, -3, -3, -3, 0, 6, 12, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
                    37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0,
                ],
                [
                    /* 5500 MHz */
                    -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -7, -2, 3,
                    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 24, 25,
                    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0,
                ],
            ];

            GainTable {
                info,
                index: 4, // active info at index 4
                abs_gain_tbl: abs_gain_tbl[index],
                table: table[index],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_table_from_recommended() {
        let _gt =
            GainTable::new_from_recommended(GainTableKind::Full, 2_000_000_000);
    }

    #[test]
    fn set_gain_entry() {
        let mut gt = GainTable::new_from_recommended(GainTableKind::Full, 0);
        let mut ge = gt.get_entry(1);
        ge.set_reg131(0);
        gt.set_entry(1, ge);
    }

    #[test]
    #[should_panic]
    fn get_gain_entry_out_of_bounds() {
        let gt = GainTable::new_from_recommended(GainTableKind::Full, 0);
        let _ = gt.get_entry(99);
    }
}
