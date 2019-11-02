use std::ops::{Add, AddAssign, Bound, Index, Range, RangeBounds, RangeInclusive, Sub, SubAssign};

use bitvec::cursor::BigEndian;
use bitvec::slice::BitSlice;

mod consts;
pub use self::consts::*;

use crate::*;

pub type Word = u16;

pub type ZBitSlice = BitSlice<BigEndian, u8>;

/// Wrapper type for a [`usize`] representing a byte address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteAddress(pub usize);

/// Wrapper type for a [`usize`] representing a bit address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitAddress(usize);

/// Wrapper type for a [`usize`] representing a [`Word`] address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WordAddress(usize);

impl WordAddress {
    /// The word address `0x0`.
    pub const ZERO: Self = Self(0);
    /// The address of the high byte of the [`Word`].
    pub fn high_byte(&self) -> ByteAddress {
        (*self).into()
    }
    /// The address of the low byte of the [`Word`].
    pub fn low_byte(&self) -> ByteAddress {
        self.high_byte() + 1
    }
    /// The address number, in words. Be careful not to make or do math with a different kind of
    /// address.
    pub fn addr(&self) -> usize {
        self.0
    }
}

impl WordAddress {
    /// The section of a story containing the header.
    const HEADER: Range<WordAddress> = WordAddress(0x00)..WordAddress(0x20);
}

impl ByteAddress {
    /// The byte address `0x0`.
    pub const ZERO: Self = Self(0);
    /// The address number, in bytes. Equivalent to `addr.0`.
    pub fn addr(&self) -> usize {
        self.0
    }
}

impl BitAddress {
    /// The bit address `0x0`.
    pub const ZERO: Self = Self(0);
    /// The address number, in bits. Be careful not to make or do math with a different kind of
    /// address.
    pub fn addr(&self) -> usize {
        self.0
    }
}

impl From<Word> for ByteAddress {
    fn from(word: Word) -> Self {
        Self(word as usize)
    }
}

impl From<Word> for BitAddress {
    fn from(word: Word) -> Self {
        ByteAddress::from(word).into()
    }
}

impl From<Word> for WordAddress {
    fn from(word: Word) -> Self {
        ByteAddress::from(word).into()
    }
}

impl Index<BitAddress> for ZMachine {
    type Output = bool;
    fn index(&self, index: BitAddress) -> &Self::Output {
        &ZBitSlice::from_slice(&self.memory)[index.0]
    }
}

impl Index<Range<BitAddress>> for ZMachine {
    type Output = ZBitSlice;
    fn index(&self, index: Range<BitAddress>) -> &Self::Output {
        &ZBitSlice::from_slice(&self.memory)[index.start.0..index.end.0]
    }
}

impl Index<RangeInclusive<BitAddress>> for ZMachine {
    type Output = ZBitSlice;
    fn index(&self, index: RangeInclusive<BitAddress>) -> &Self::Output {
        &ZBitSlice::from_slice(&self.memory)[index.start().0..=index.end().0]
    }
}

impl Index<ByteAddress> for ZMachine {
    type Output = u8;
    fn index(&self, index: ByteAddress) -> &Self::Output {
        &self.memory[index.0]
    }
}

impl Index<Range<ByteAddress>> for ZMachine {
    type Output = [u8];
    fn index(&self, index: Range<ByteAddress>) -> &Self::Output {
        &self.memory[index.start.0..index.end.0]
    }
}

impl Index<RangeInclusive<ByteAddress>> for ZMachine {
    type Output = [u8];
    fn index(&self, index: RangeInclusive<ByteAddress>) -> &Self::Output {
        &self.memory[index.start().0..=index.end().0]
    }
}

impl ZMachine {
    pub fn word(&self, index: WordAddress) -> Word {
        let addr = ByteAddress::from(index);
        let word = &self.memory[addr.0..=(addr + 1).0];
        Word::from_be_bytes([word[0], word[1]])
    }
}

impl From<BitAddress> for ByteAddress {
    fn from(addr: BitAddress) -> Self {
        Self(addr.0 / 8)
    }
}

impl From<WordAddress> for ByteAddress {
    fn from(addr: WordAddress) -> Self {
        Self(addr.0 * 2)
    }
}

impl From<ByteAddress> for BitAddress {
    fn from(addr: ByteAddress) -> Self {
        Self(addr.0 * 8)
    }
}

impl From<WordAddress> for BitAddress {
    fn from(addr: WordAddress) -> Self {
        Self(addr.0 * 16)
    }
}

impl From<BitAddress> for WordAddress {
    fn from(addr: BitAddress) -> Self {
        Self(addr.0 / 16)
    }
}

impl From<ByteAddress> for WordAddress {
    fn from(addr: ByteAddress) -> Self {
        Self(addr.0 / 2)
    }
}

impl Add<usize> for ByteAddress {
    type Output = ByteAddress;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for ByteAddress {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Sub<usize> for ByteAddress {
    type Output = ByteAddress;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<usize> for ByteAddress {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Add<usize> for BitAddress {
    type Output = BitAddress;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for BitAddress {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Sub<usize> for BitAddress {
    type Output = BitAddress;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<usize> for BitAddress {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Add<usize> for WordAddress {
    type Output = WordAddress;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for WordAddress {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Sub<usize> for WordAddress {
    type Output = WordAddress;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<usize> for WordAddress {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}
