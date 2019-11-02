#![feature(crate_visibility_modifier)]
#![allow(dead_code)]

#[cfg(test)]
mod tests;

mod bits;
pub use self::bits::*;
mod memory;
pub use self::memory::*;
mod vm;
pub use self::vm::*;
mod meta;
pub use self::meta::*;
mod util;
pub use self::util::*;
mod text;
pub use self::text::*;
pub mod constants;

use std::path::Path;
use std::fs::File;
use std::io::{Error as IoError, Read};
use failure::Fail;

/// An implementation of a [Z-Machine](https://en.wikipedia.org/wiki/Z-machine) with a loaded story.
pub struct ZMachine {
    crate memory: Vec<u8>,
}

impl ZMachine {
    /// Creates a new Z-machine instance from a story file already loaded in memory.
    pub fn new(file: impl Into<Vec<u8>>) -> LoadResult<Self> {
        let vec = file.into();
        if vec.len() < 64 {
            return Err(LoadError::TooSmall(vec.len()))
        }
        Ok(Self {
            memory: vec,
        })
    }
    /// Utility function for reading from a filename and passing the contents to [`Self::new`].
    pub fn from_file(path: impl AsRef<Path>) -> LoadResult<Self> {
        let path = path.as_ref();
        let mut vec = Vec::new();
        File::open(path)?.read_to_end(&mut vec)?;
        Self::new(vec)
    }
    //todo version-specific header sizing
    /// Gets the length of the story in bytes.
    pub fn len_bytes(&self) -> usize {
        self.memory.len()
    }
    /// Gets the length of the story in bits.
    pub fn len_bits(&self) -> usize {
        self.memory.len() * 8
    }
    /// Gets the length of the story in [`Word`]s.
    pub fn len_words(&self) -> usize {
        self.memory.len() / 2
    }
}

/// Errors that can occur during loading a story.
#[derive(Debug, Fail)]
pub enum LoadError {
    /// An error during IO. Only used with [`ZMachine::from_file`].
    #[fail(display = "IO error: {}", _0)]
    IoError(#[cause] IoError),
    /// An error to do with the story's size. A story without a header (64 bytes) cannot be read.
    #[fail(display = "Story is too small (was {} bytes, must be at least 64)", _0)]
    TooSmall(usize),
    /// An unknown error of some other kind.
    #[fail(display = "Unknown error")]
    Unknown,
}

impl From<IoError> for LoadError {
    fn from(err: IoError) -> Self {
        LoadError::IoError(err)
    }
}

pub type LoadResult<T> = Result<T, LoadError>;
