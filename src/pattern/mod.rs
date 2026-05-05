use std::path::PathBuf;
use crate::pattern::{stitch_buffer::StitchBuffer, stitch_palette::StitchPalette};

pub mod io;
pub mod stitch_palette;
pub mod stitch_buffer;
mod color_palette;

/// An incomplete draft form of a new pattern.
#[derive(Debug, Clone)]
pub struct PatternDraft {
    pub width: u16,
    pub height: u16,
    pub path: Option<PathBuf>,
}
impl PatternDraft {
    pub fn new() -> Self {
        Self {
            width: 30,
            height: 30,
            path: None,
        }
    }
}

/// A cross stitch pattern.
#[derive(Debug, PartialEq)]
pub struct Pattern {
    /// Width of the pattern grid.
    pub width: u16,
    /// Height of the pattern grid.
    pub height: u16,
    pub palette: StitchPalette,
    pub primary_grid: StitchBuffer,
}
impl Pattern {
    /// Takes the draft form of a new pattern and turns it into a proper pattern.
    pub fn from_draft(draft: &PatternDraft) -> Self {
        Self {
            width: draft.width,
            height: draft.height,
            palette: StitchPalette::new(),
            primary_grid: StitchBuffer::with_size(draft.width, draft.height),
        }
    }

    pub fn new(
        width: u16,
        height: u16,
        primary_grid: StitchBuffer,
    ) -> Self {
        Self {
            width,
            height,
            palette: StitchPalette::new(),
            primary_grid,
        }
    }
}
