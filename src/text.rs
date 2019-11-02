use crate::*;

/// An abbreviation identifier for a Z-string. References a Z-string addressed in the abbreviation
/// table.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZStringAbbrv(u8);

impl ZStringAbbrv {
    /// Creates a new abbreviation. Valid numbers are in `0..96`; a higher index will return
    /// [`None`].
    /// # Note
    /// 96 is the exclusive maximum number for abbreviations, but story versions before 3 only
    /// support abbreviations up to an exclusive maximum of 32. You can use
    /// [`ZMachine::is_abbrv_valid`] to check if an abbreviation is valid for a story.
    pub fn new(idx: u8) -> Option<Self> {
        if idx >= 96 {
            None
        } else {
            Some(Self(idx))
        }
    }
    /// This abbreviation's index (i.e. the word-sized offset in the abbreviation table).
    pub fn idx(&self) -> u8 {
        self.0
    }
}

/// A reference to a Z-string. A Z-string is like a string, but more annoying.
pub struct ZStr<'a> {
    loc: &'a u8,
}

impl ZMachine {
    /// Checks if a Z-string abbreviation is valid. An abbreviation is always valid if the story's
    /// version is 3 or greater, but if the story version is below 3, only abbreviations with
    /// indexes below 32 are valid.
    pub fn is_abbrv_valid(&self, abbrv: ZStringAbbrv) -> bool {
        match self.version() {
            Version::V1 | Version::V2 => abbrv.0 < 32,
            _ => true,
        }
    }
    /// Gets a Z-string referenced by a particular abbreviation. Panics if
    /// [`is_abbrv_valid`](ZMachine::is_abbrv_valid) returns false for this abbreviation.
    pub fn get_abbrvd_zstr(&self, abbrv: ZStringAbbrv) -> ZStr {
        if !self.is_abbrv_valid(abbrv) {
            panic!("Invalid Z-string abbreviation {}", abbrv.0);
        }
        let abbrv_table = self.get_abbreviations_table_base();
        let abbrv_table_idx = abbrv_table + abbrv.0 as usize;
        let abbrv_addr = ByteAddress::from(self.word(abbrv_table_idx) * 2);
        ZStr {
            loc: &self[abbrv_addr],
        }
    }
    /// Gets a Z-string at a particular address in memory, if there is one. If it is not a valid
    /// Z-string starting point, returns [`None`].
    pub fn get_zstr(&self, addr: WordAddress) -> Option<ZStr> {
        //todo validation
        Some(ZStr { loc: &self[ByteAddress::from(addr)] })
    }
}
