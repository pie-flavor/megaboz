use super::*;

impl BitAddress {
    pub const STATUS_LINE_AVAILABLE: Self = Self(0xC);
    pub const SCREEN_SPLIT_AVAILABLE: Self = Self(0xD);
    pub const VARIABLE_PITCH_FONT_DEFAULT: Self = Self(0xE);
    pub const COLORS_AVAILABLE: Self = Self(0x8);
    pub const PICTURES_AVAILABLE: Self = Self(0x9);
    pub const BOLD_AVAILABLE: Self = Self(0xA);
    pub const ITALIC_AVAILABLE: Self = Self(0xB);
    pub const FIXED_SPACE_AVAILABLE: Self = Self(0xC);
    pub const SOUND_EFFECT_AVAILABLE: Self = Self(0xD);
    pub const TIMED_KEYBOARD_AVAILABLE: Self = Self(0xF);
    pub const TRANSCRIPTING_ON: Self = Self(0x70);
    pub const FORCE_FIXED_PITCH: Self = Self(0x71);
    pub const SCREEN_REDRAW_REQUESTED: Self = Self(0x72);
    pub const PICTURES_DESIRED: Self = Self(0x73);
    pub const UNDO_DESIRED: Self = Self(0x74);
    pub const MOUSE_DESIRED: Self = Self(0x75);
    pub const COLORS_DESIRED: Self = Self(0x76);
    pub const SOUNDS_DESIRED: Self = Self(0x77);
    pub const MENUS_DESIRED: Self = Self(0x78);
    pub const HEADER_EXT_TRANSPARENCY_DESIRED: Self = Self(0x40);
    pub const STATUS_LINE: Self = Self(0x9);
    pub const TWO_DISKS: Self = Self(0xA);
}

impl WordAddress {
    pub const HIGH_MEMORY_LOCATION: Self = Self(0x2);
    pub const INITIAL_PC_LOCATION: Self = Self(0x3);
    pub const MAIN_LOCATION: Self = Self(0x3);
    pub const DICTIONARY_LOCATION: Self = Self(0x4);
    pub const OBJECT_TABLE_LOCATION: Self = Self(0x5);
    pub const GLOBAL_VARIABLE_TABLE_LOCATION: Self = Self(0x6);
    pub const STATIC_MEMORY_LOCATION: Self = Self(0x7);
    pub const ABBREVIATIONS_LOCATION: Self = Self(0xC);
    pub const FILE_LENGTH: Self = Self(0xD);
    pub const FILE_CHECKSUM: Self = Self(0xE);
    pub const SCREEN_WIDTH_UNITS: Self = Self(0x11);
    pub const SCREEN_HEIGHT_UNITS: Self = Self(0x12);
    pub const ROUTINES_OFFSET: Self = Self(0x14);
    pub const STATIC_STRINGS_OFFSET: Self = Self(0x15);
    pub const TERMINATING_CHARACTERS_TABLE_LOCATION: Self = Self(0x17);
    pub const OUTPUT_STREAM_3_WIDTH_TOTAL_PIXELS: Self = Self(0x18);
    pub const ALPHABET_TABLE_ADDRESS: Self = Self(0x1A);
    pub const HEADER_EXTENSION_TABLE_ADDRESS: Self = Self(0x1B);
    pub const HEADER_EXT_SIZE: Self = Self(0x0);
    pub const HEADER_EXT_MOUSE_X: Self = Self(0x1);
    pub const HEADER_EXT_MOUSE_Y: Self = Self(0x2);
    pub const HEADER_EXT_UNICODE_TRANSLATION_TABLE_LOCATION: Self = Self(0x3);
    pub const HEADER_EXT_DEFAULT_FOREGROUND_COLOR: Self = Self(0x5);
    pub const HEADER_EXT_DEFAULT_BACKGROUND_COLOR: Self = Self(0x6);
}

impl ByteAddress {
    pub const STORY_VERSION: Self = Self(0x0);
    pub const INTERPRETER_NUMBER: Self = Self(0x1E);
    pub const INTERPRETER_VERSION: Self = Self(0x1F);
    pub const SCREEN_HEIGHT: Self = Self(0x20);
    pub const SCREEN_WIDTH_CHARS: Self = Self(0x21);
    pub const FONT_WIDTH_UNITS_V5: Self = Self(0x26);
    pub const FONT_WIDTH_UNITS_V6: Self = Self(0x27);
    pub const FONT_HEIGHT_UNITS_V5: Self = Self(0x27);
    pub const FONT_HEIGHT_UNITS_V6: Self = Self(0x26);
    pub const DEFAULT_BACKGROUND_COLOR: Self = Self(0x2C);
    pub const DEFAULT_FOREGROUND_COLOR: Self = Self(0x2D);
    pub const STANDARD_REVISION_MAJOR: Self = Self(0x32);
    pub const STANDARD_REVISION_MINOR: Self = Self(0x33);
}

impl BitAddress {
    pub const HEADER_EXT_UNUSED_FLAGS3: Range<Self> = Self(0x41)..Self(0x48);
}
