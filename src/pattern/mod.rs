use std::path::PathBuf;


pub mod pattern_file;

/// An incomplete draft form of a new pattern.
#[derive(Debug, Clone)]
pub struct PatternDraft {
    pub width: u16,
    pub height: u16,
}
impl PatternDraft {
    pub fn new() -> Self {
        Self {
            width: 30,
            height: 30,
        }
    }
}

/// A cross stitch pattern.
#[derive(Debug)]
pub struct Pattern {
    /// Width of the pattern grid.
    pub width: u16,
    /// Height of the pattern grid.
    pub height: u16,
    /// Path to where the pattern is saved to / loaded from.
    pub path: Option<PathBuf>,
}
impl Pattern {
    /// Takes the draft form of a new pattern and turns it into a proper pattern.
    pub fn from_draft(draft: &PatternDraft) -> Self {
        Self {
            width: draft.width,
            height: draft.height,
            path: None,
        }
    }
}
