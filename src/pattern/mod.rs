use std::path::PathBuf;
use crate::pattern::stitch_palette::{StitchPalette, StitchPaletteIndex};

pub mod io;
mod stitch_palette;
mod stitch_grid;
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
#[derive(Debug)]
pub struct Pattern {
    /// Width of the pattern grid.
    pub width: u16,
    /// Height of the pattern grid.
    pub height: u16,
    /// Path to where the pattern is saved to / loaded from.
    pub path: Option<PathBuf>,
    pub palette: StitchPalette,
}
impl Pattern {
    /// Takes the draft form of a new pattern and turns it into a proper pattern.
    pub fn from_draft(draft: &PatternDraft) -> Self {
        Self {
            width: draft.width,
            height: draft.height,
            path: draft.path.clone(),
            palette: StitchPalette::new(),
        }
    }

    pub fn from_file(
        width: u16,
        height: u16,
        path: PathBuf,
    ) -> Self {
        Self {
            width,
            height,
            path: Some(path),
            palette: StitchPalette::new(),
        }
    }
}
