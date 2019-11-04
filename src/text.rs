use crate::*;
use std::char;

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
    /// Returns a Z-string referenced by a particular abbreviation. Panics if
    /// [`is_abbrv_valid`](ZMachine::is_abbrv_valid) returns false for this abbreviation.
    pub fn abbrvd_zstring(&self, abbrv: ZStringAbbrv) -> String {
        let mut string = String::new();
        self.copy_abbrvd_zstring(abbrv, &mut string);
        string
    }
    /// Copies a Z-string referenced by a particular abbreviation into a string buffer.
    pub fn copy_abbrvd_zstring(&self, abbrv: ZStringAbbrv, str: &mut String) {
        assert!(
            self.is_abbrv_valid(abbrv),
            "Invalid Z-string abbreviation {}",
            abbrv.0
        );
        let abbrv_table = self.abbreviations_table_base();
        let abbrv_table_idx = abbrv_table + (abbrv.0 as usize) * 2;
        let abbrv_addr = ByteAddress::from(self.word(abbrv_table_idx) * 2);
        assert!(
            self.copy_zstring(abbrv_addr.into(), str),
            "Invalid Z-string at abbreviation {}",
            abbrv.0
        );
    }
    /// Returns a Z-string at a particular address in memory, if there is one. If it is not a valid
    /// Z-string starting point, returns [`None`].
    pub fn zstring(&self, addr: ByteAddress) -> Option<String> {
        let mut str = String::new();
        if self.copy_zstring(addr, &mut str) {
            Some(str)
        } else {
            None
        }
    }
    /// Copies a Z-string at a particular address in memory into a string buffer, if there is one.
    /// Returns true if the copy was successful (i.e. a valid encoding) and false if it wasn't.
    pub fn copy_zstring(&self, addr: ByteAddress, string: &mut String) -> bool {
        let mut addr = addr;
        fn zchar_to_byte(bits: &BEBitSlice<Word>) -> u8 {
            ((bits[0] as u8) << 4)
                | ((bits[1] as u8) << 3)
                | ((bits[2] as u8) << 2)
                | ((bits[3] as u8) << 1)
                | (bits[4] as u8)
        }
        let alphabet = self.alphabet();
        let mut state = ZStringState::Unset;
        let mut mode = AlphabetMode::Lowercase;
        loop {
            let word = self.word(addr);
            let bits = BEBitSlice::from_element(&word);
            let is_end = bits[0];
            let zchar1 = zchar_to_byte(&bits[1..6]);
            let zchar2 = zchar_to_byte(&bits[6..11]);
            let zchar3 = zchar_to_byte(&bits[11..16]);
            let zchars = [zchar1, zchar2, zchar3];
            for &zchar in &zchars {
                self.decode_zchar(zchar, &mut mode, &mut state, &alphabet, string);
            }
            if is_end {
                break;
            }
            addr += 2;
        }
        true
    }
    fn decode_zchar(
        &self,
        current_zchar: u8,
        alphabet_mode: &mut AlphabetMode,
        state: &mut ZStringState,
        alphabet: &Alphabet,
        string: &mut String,
    ) -> bool {
        fn rotate_up(mode: AlphabetMode) -> AlphabetMode {
            match mode {
                AlphabetMode::Lowercase => AlphabetMode::Uppercase,
                AlphabetMode::Uppercase => AlphabetMode::Symbol,
                AlphabetMode::Symbol => AlphabetMode::Lowercase,
            }
        }
        fn rotate_down(mode: AlphabetMode) -> AlphabetMode {
            match mode {
                AlphabetMode::Lowercase => AlphabetMode::Symbol,
                AlphabetMode::Uppercase => AlphabetMode::Lowercase,
                AlphabetMode::Symbol => AlphabetMode::Uppercase,
            }
        }
        let mut current_mode = *alphabet_mode;
        let (printable, mut new_state) = match *state {
            ZStringState::TenBitHigh => (false, ZStringState::TenBitLow(current_zchar)),
            ZStringState::TenBitLow(prev) => {
                let zscii = ((prev as u16) << 5) | current_zchar as u16;
                if let Some(ch) = self.zscii_char(zscii) {
                    string.push(ch);
                }
                (false, ZStringState::Unset)
            }
            ZStringState::Abbreviation(section) => {
                let abbrv = ZStringAbbrv((section - 1) * 32 + current_zchar);
                self.copy_abbrvd_zstring(abbrv, string);
                (false, ZStringState::Unset)
            }
            ZStringState::ModeShift(mode) => {
                current_mode = mode;
                (true, ZStringState::Unset)
            }
            ZStringState::Unset => (true, ZStringState::Unset),
        };
        if printable {
            new_state = match current_zchar {
                0 => {
                    string.push(' ');
                    new_state
                }
                1 => {
                    if self.version() == Version::V1 {
                        string.push('\n');
                        new_state
                    } else {
                        ZStringState::Abbreviation(1)
                    }
                }
                2 => {
                    if self.version() > Version::V2 {
                        ZStringState::Abbreviation(2)
                    } else {
                        ZStringState::ModeShift(rotate_up(*alphabet_mode))
                    }
                }
                3 => {
                    if self.version() > Version::V2 {
                        ZStringState::Abbreviation(3)
                    } else {
                        ZStringState::ModeShift(rotate_down(*alphabet_mode))
                    }
                }
                4 => {
                    if self.version() > Version::V2 {
                        ZStringState::ModeShift(rotate_up(*alphabet_mode))
                    } else {
                        *alphabet_mode = rotate_up(*alphabet_mode);
                        new_state
                    }
                }
                5 => {
                    if self.version() > Version::V2 {
                        ZStringState::ModeShift(rotate_down(*alphabet_mode))
                    } else {
                        *alphabet_mode = rotate_down(*alphabet_mode);
                        new_state
                    }
                }
                6 if current_mode == AlphabetMode::Symbol => ZStringState::TenBitHigh,
                _ => {
                    string.push(alphabet.letter_for_zchar(current_zchar, current_mode));
                    new_state
                }
            };
        }
        *state = new_state;
        true
    }
    /// Returns the alphabet in use by this story.
    pub fn alphabet(&self) -> Alphabet {
        match self.version() {
            Version::V1 => Alphabet {
                lower: &DEFAULT_LOWERCASE_ALPHABET,
                upper: &DEFAULT_UPPERCASE_ALPHABET,
                symbol: &DEFAULT_SYMBOL_ALPHABET_V1,
            },
            Version::V2 | Version::V3 | Version::V4 => Alphabet::default(),
            _ => {
                let word = self.word(ByteAddress::DICTIONARY_LOCATION);
                if word == 0 {
                    Alphabet::default()
                } else {
                    let addr = ByteAddress::from(word);
                    let bytes = &self[addr..(addr + 78)];
                    Alphabet {
                        lower: &bytes[0..26],
                        upper: &bytes[26..52],
                        symbol: &bytes[52..78],
                    }
                }
            }
        }
    }
    /// Returns the unicode table in use by this story.
    pub fn unicode_table(&self) -> UnicodeTable {
        if self.version() < Version::V5 {
            UnicodeTable::default()
        } else {
            let word = self.word(ByteAddress::HEADER_EXTENSION_TABLE_ADDRESS);
            if word == 0 {
                UnicodeTable::default()
            } else {
                let ext_addr = ByteAddress::from(word);
                let unicode_addr_addr =
                    ext_addr + ByteAddress::HEADER_EXT_UNICODE_TRANSLATION_TABLE_LOCATION;
                let unicode_addr = self.word(unicode_addr_addr);
                if unicode_addr == 0 {
                    UnicodeTable::default()
                } else {
                    let unicode_addr = ByteAddress::from(unicode_addr);
                    let len = self[unicode_addr];
                    let table = &self[(unicode_addr + 1)..(unicode_addr + 1 + len as usize)];
                    UnicodeTable { table }
                }
            }
        }
    }
    /// Returns the base address of the dictionary (i.e. the start of the table header).
    pub fn dictionary_base(&self) -> ByteAddress {
        self.word(ByteAddress::DICTIONARY_LOCATION).into()
    }
    fn word_separators_len(&self) -> usize {
        self[self.dictionary_base()] as usize
    }
    /// Returns a list of all the word separator characters (excluding space).
    pub fn word_separators(&self) -> Vec<char> {
        let mut vec = Vec::new();
        self.copy_word_separators(&mut vec);
        vec
    }
    /// Copies the word separator characters (excluding space) into a provided buffer.
    pub fn copy_word_separators(&self, buf: &mut Vec<char>) {
        let len = self.word_separators_len();
        buf.reserve(len);
        let separator_base = self.dictionary_base() + 1;
        for x in 0..len {
            let separator_addr = separator_base + x;
            let zscii = self[separator_addr] as u16;
            if let Some(ch) = self.zscii_char(zscii) {
                buf.push(ch);
            }
        }
    }
    /// Converts a ZSCII character into a `char`.
    pub fn zscii_char(&self, zscii: u16) -> Option<char> {
        match zscii {
            9 => {
                if self.version() == Version::V6 {
                    Some('\t')
                } else {
                    None
                }
            }
            11 => {
                if self.version() == Version::V6 {
                    Some(' ') // sentence space
                } else {
                    None
                }
            }
            13 => Some('\n'),
            32..=126 => Some(zscii as u8 as char),
            155..=251 => Some(self.unicode_table().zscii_to_char(zscii as u8)),
            _ => None,
        }
    }
    fn dictionary_words_base(&self) -> ByteAddress {
        self.dictionary_base() + self.word_separators_len() + 2
    }
    /// Returns the number of words in the dictionary.
    pub fn dictionary_len(&self) -> usize {
        self.word(self.dictionary_words_base()) as usize
    }
    fn dictionary_entry_size(&self) -> usize {
        self[self.dictionary_base() + self.word_separators_len() + 1] as usize
    }
    /// Returns a dictionary word at a particular index. Panics if the index is out of bounds
    /// ([`dictionary_len`](ZMachine::dictionary_len))
    pub fn dictionary_word(&self, idx: usize) -> String {
        let mut string = String::new();
        self.copy_dictionary_word(idx, &mut string);
        string
    }
    /// Copies a dictionary word at a particular index into the provided buffer. Panics if the index
    /// is out of bounds ([`dictionary_len`](ZMachine::dictionary_len)
    pub fn copy_dictionary_word(&self, idx: usize, string: &mut String) {
        assert!(
            idx < self.dictionary_len(),
            "Dictionary index {} out of bounds",
            idx
        );
        let word_sz = self.dictionary_entry_size();
        let offset = idx * word_sz + 2;
        self.copy_zstring(
            (ByteAddress::from(self.dictionary_words_base()) + offset).into(),
            string,
        );
    }
    /// Returns a list of all words in the dictionary.
    pub fn dictionary_words(&self) -> Vec<String> {
        let len = self.dictionary_len();
        let mut vec = Vec::with_capacity(len);
        let word_sz = self.dictionary_entry_size();
        let start = ByteAddress::from(self.dictionary_words_base() + 2);
        for x in 0..len {
            vec.push(
                self.zstring(start + x * word_sz)
                    .unwrap_or_else(|| panic!("Invalid Z-string at dictionary index {}", x)),
            );
        }
        vec
    }
}

#[derive(Debug, Copy, Clone)]
enum ZStringState {
    TenBitHigh,
    TenBitLow(u8),
    ModeShift(AlphabetMode),
    Abbreviation(u8),
    Unset,
}

static DEFAULT_LOWERCASE_ALPHABET: [u8; 26] = *b"abcdefghijklmnopqrstuvwxyz";
static DEFAULT_UPPERCASE_ALPHABET: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static DEFAULT_SYMBOL_ALPHABET_V1: [u8; 26] = *br#" 0123456789.,!?_#'"/\<-:()"#;
static DEFAULT_SYMBOL_ALPHABET: [u8; 26] = *b" \n0123456789.,!?_#'\"/\\-:()";
static DEFAULT_UNICODE_TABLE: [u8; 138] = [
    0x0, 0xe4, 0x0, 0xf6, 0x0, 0xfc, 0x0, 0xc4, 0x0, 0xd6, 0x0, 0xdc, 0x0, 0xdf, 0x0, 0xbb, 0x0,
    0xab, 0x0, 0xeb, 0x0, 0xef, 0x0, 0xff, 0x0, 0xcb, 0x0, 0xcf, 0x0, 0xe1, 0x0, 0xe9, 0x0, 0xed,
    0x0, 0xf3, 0x0, 0xfa, 0x0, 0xfd, 0x0, 0xc1, 0x0, 0xc9, 0x0, 0xcd, 0x0, 0xd3, 0x0, 0xda, 0x0,
    0xdd, 0x0, 0xe0, 0x0, 0xe8, 0x0, 0xec, 0x0, 0xf2, 0x0, 0xf9, 0x0, 0xc0, 0x0, 0xc8, 0x0, 0xcc,
    0x0, 0xd2, 0x0, 0xd9, 0x0, 0xe2, 0x0, 0xea, 0x0, 0xee, 0x0, 0xf4, 0x0, 0xfb, 0x0, 0xc2, 0x0,
    0xca, 0x0, 0xce, 0x0, 0xd4, 0x0, 0xdb, 0x0, 0xe5, 0x0, 0xc5, 0x0, 0xf8, 0x0, 0xd8, 0x0, 0xe3,
    0x0, 0xf1, 0x0, 0xf5, 0x0, 0xc3, 0x0, 0xd1, 0x0, 0xd5, 0x0, 0xe6, 0x0, 0xc6, 0x0, 0xe7, 0x0,
    0xc7, 0x0, 0xfe, 0x0, 0xf0, 0x0, 0xde, 0x0, 0xd0, 0x0, 0xa3, 0x1, 0x53, 0x1, 0x52, 0x0, 0xa1,
    0x0, 0xbf,
];

/// The text mode used when indexing an [`Alphabet`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlphabetMode {
    /// Lowercase mode.
    Lowercase,
    /// Uppercase mode.
    Uppercase,
    /// Number/symbol mode.
    Symbol,
}

impl AlphabetMode {
    /// A convenience array of all the alphabet values.
    pub const VALUES: [AlphabetMode; 3] = [
        AlphabetMode::Lowercase,
        AlphabetMode::Uppercase,
        AlphabetMode::Symbol,
    ];
}

/// The alphabet that a story uses for encoding Z-strings.
#[derive(Debug, Copy, Clone)]
pub struct Alphabet<'a> {
    lower: &'a [u8],
    upper: &'a [u8],
    symbol: &'a [u8],
}

impl Alphabet<'_> {
    /// Returns a letter at a particular index in a particular alphabet mode.
    pub fn letter_at_index(&self, idx: u8, mode: AlphabetMode) -> char {
        (match mode {
            AlphabetMode::Lowercase => self.lower[idx as usize],
            AlphabetMode::Uppercase => self.upper[idx as usize],
            AlphabetMode::Symbol => self.symbol[idx as usize],
        }) as char
    }
    /// Returns a letter for a particular z-char in a particular alphabet mode.
    pub fn letter_for_zchar(&self, zchar: u8, mode: AlphabetMode) -> char {
        let idx = zchar.checked_sub(6).unwrap_or_else(|| {
            panic!(
                "Invalid alphabet ZSCII character {}, must be in range 6..32",
                zchar
            )
        });
        self.letter_at_index(idx, mode)
    }
}

impl Default for Alphabet<'_> {
    fn default() -> Self {
        Self {
            upper: &DEFAULT_UPPERCASE_ALPHABET,
            lower: &DEFAULT_LOWERCASE_ALPHABET,
            symbol: &DEFAULT_SYMBOL_ALPHABET,
        }
    }
}

/// The Unicode table used for ZSCII characters over 154.
#[derive(Debug, Copy, Clone)]
pub struct UnicodeTable<'a> {
    table: &'a [u8],
}

impl UnicodeTable<'_> {
    /// Returns a `char` at a particular index in the table.
    pub fn char_at_index(&self, idx: u8) -> char {
        let idx = idx as usize;
        let high = self.table[idx];
        let low = self.table[idx + 1];
        char::from_u32(u16::from_be_bytes([high, low]) as u32)
            .unwrap_or_else(|| panic!("Invalid char at unicode table index {}", idx))
    }
    /// Converts a ZSCII character to a `char`.
    pub fn zscii_to_char(&self, zscii: u8) -> char {
        let idx = zscii.checked_sub(155).unwrap_or_else(|| {
            panic!(
                "Invalid extended ZSCII char {}, must be in range 155..252",
                zscii
            )
        });
        assert!(
            idx < self.table.len() as u8,
            "Invalid extended ZSCII char {} for this story, current unicode table spans 155..{}",
            zscii,
            self.table.len()
        );
        self.char_at_index(idx)
    }
}

impl Default for UnicodeTable<'_> {
    fn default() -> Self {
        Self {
            table: &DEFAULT_UNICODE_TABLE,
        }
    }
}
