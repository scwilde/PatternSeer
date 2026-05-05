use crate::pattern::color_palette::{ColorPaletteIndex, ColorPalette};

#[repr(u16)]
#[derive(Debug, PartialEq)]
enum StitchType {
    Empty = 0,
    FullCross = 1,

    Unknown(u16) = 65535,
}

#[derive(Debug, PartialEq)]
struct Thread {
    color_index: ColorPaletteIndex,
    ct: u8,
}

#[derive(Debug, PartialEq)]
struct Stitch {
    stitch_type: StitchType,
    threads: Vec<Thread>,
}

#[derive(Debug, PartialEq)]
pub struct StitchPalette {
    color_palette: ColorPalette,
    stitches: Vec<Stitch>,
}
impl StitchPalette {
    pub fn new() -> Self {
        Self {
            color_palette: ColorPalette::new(),
            stitches: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StitchPaletteIndex(pub u16);
impl StitchPaletteIndex {
    pub fn new(index: u16) -> Self {
        Self(index)
    }

    pub fn to_le_bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }

    pub fn from_le_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
}
