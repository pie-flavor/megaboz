use crate::*;
use std::cmp;
use std::ops::Range;

impl ZMachine {
    /// Gets the range of memory classified 'dynamic'.
    pub fn dynamic_memory_range(&self) -> Range<WordAddress> {
        WordAddress::ZERO..WordAddress::from(self.word(WordAddress::STATIC_MEMORY_LOCATION) - 1)
    }
    /// Gets the range of memory classified 'static'.
    pub fn static_memory_range(&self) -> Range<WordAddress> {
        let end = cmp::min(self.len_bytes(), 0x10000);
        WordAddress::from(self.word(WordAddress::STATIC_MEMORY_LOCATION))..WordAddress::from(ByteAddress(end))
    }
    /// Gets the range of memory classified 'high'.
    pub fn high_memory_range(&self) -> Range<WordAddress> {
        WordAddress::from(self.word(WordAddress::HIGH_MEMORY_LOCATION))
            ..WordAddress::from(ByteAddress(self.len_bytes()))
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
    pub fn write_word(&mut self, address: WordAddress, word: Word) {
        let mut bytes = word.to_be_bytes();
        let byte_address = ByteAddress::from(address);
        self.memory[byte_address.0..=(byte_address + 1).0].swap_with_slice(&mut bytes);
    }
    pub fn get_abbreviations_table_base(&self) -> WordAddress {
        WordAddress::from(ByteAddress::from(self.word(WordAddress::ABBREVIATIONS_LOCATION)))
    }
}
