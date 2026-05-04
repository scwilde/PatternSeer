use crate::pattern::color_palette::{ColorPaletteIndex, ColorPalette};

#[repr(u16)]
#[derive(Debug)]
enum StitchType {
    Empty = 0,
    FullCross = 1,

    Unknown(u16) = 65535,
}

#[derive(Debug)]
struct Thread {
    /// Index into a `ColorPalette`
    color_index: ColorPaletteIndex,
    ct: u8,
}

#[derive(Debug)]
struct Stitch {
    stitch_type: StitchType,
    threads: Vec<Thread>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct StitchPaletteIndex(u16);
