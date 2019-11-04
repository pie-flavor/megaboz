use crate::*;
use std::cmp;
use std::ops::Range;

impl ZMachine {
    /// Gets the range of memory classified 'dynamic'.
    pub fn dynamic_memory_range(&self) -> Range<ByteAddress> {
        ByteAddress::ZERO..ByteAddress::from(self.word(ByteAddress::STATIC_MEMORY_LOCATION) - 1)
    }
    /// Gets the range of memory classified 'static'.
    pub fn static_memory_range(&self) -> Range<ByteAddress> {
        let end = cmp::min(self.len_bytes(), 0x10000);
        ByteAddress::from(self.word(ByteAddress::STATIC_MEMORY_LOCATION))..ByteAddress(end)
    }
    /// Gets the range of memory classified 'high'.
    pub fn high_memory_range(&self) -> Range<ByteAddress> {
        ByteAddress::from(self.word(ByteAddress::HIGH_MEMORY_LOCATION))
            ..ByteAddress(self.len_bytes())
    }
    /// Writes a byte at the specified address.
    pub fn write_byte(&mut self, address: ByteAddress, byte: u8) {
        self.memory[address.0] = byte;
    }
    /// Writes a bit at the specified address.
    pub fn write_bit(&mut self, address: BitAddress, bit: bool) {
        ZBitSlice::from_slice_mut(&mut self.memory).set(address.addr(), bit);
    }
    /// Writes a [`Word`] at the specified address.
    pub fn write_word(&mut self, address: ByteAddress, word: Word) {
        let mut bytes = word.to_be_bytes();
        self.memory[address.0..=(address + 1).0].swap_with_slice(&mut bytes);
    }
    pub fn bit_range(&self, range: Range<BitAddress>) -> &ZBitSlice {
        &ZBitSlice::from_slice(&self.memory)[range.start.addr()..range.end.addr()]
    }
    pub fn get_abbreviations_table_base(&self) -> ByteAddress {
        ByteAddress::from(self.word(ByteAddress::ABBREVIATIONS_LOCATION))
    }
}
