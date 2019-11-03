use crate::*;

/// The version of a story.
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Version {
    /// Version 1.
    V1,
    /// Version 2.
    V2,
    /// Version 3.
    V3,
    /// Version 4.
    V4,
    /// Version 5.
    V5,
    /// Version 6.
    V6,
}

impl Version {
    pub const VALUES: [Version; 6] = [
        Version::V1,
        Version::V2,
        Version::V3,
        Version::V4,
        Version::V5,
        Version::V6,
    ];
}

/// The status line that the game displays.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusLine {
    /// `score/turns`
    ScoreTurns,
    /// `hours:mins`
    HoursMins,
}

impl StatusLine {
    pub const VALUES: [StatusLine; 2] = [StatusLine::ScoreTurns, StatusLine::HoursMins];
}

impl ZMachine {
    /// Gets the version of this story.
    pub fn version(&self) -> Version {
        Version::VALUES[self[ByteAddress::STORY_VERSION] as usize - 1]
    }
    /// Gets what the status line should display, or [`None`] if not applicable.
    pub fn status_line(&self) -> Option<StatusLine> {
        if self.version() <= Version::V3 {
            Some(
                if self.version() == Version::V3 && self[BitAddress::STATUS_LINE] {
                    StatusLine::HoursMins
                } else {
                    StatusLine::ScoreTurns
                },
            )
        } else {
            None
        }
    }
    /// Gets whether the story is in two-disk mode.
    pub fn is_two_disks(&self) -> bool {
        self[BitAddress::TWO_DISKS]
    }
}
