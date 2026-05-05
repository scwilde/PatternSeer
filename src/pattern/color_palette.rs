use crate::utils;


#[derive(Debug, PartialEq)]
struct ThreadColor {
    brand: String,
    brand_code: String,
    color_code: String,
    rgb_color: utils::Color,
}

#[derive(Debug, PartialEq)]
pub struct ColorPalette {
    colors: Vec<ThreadColor>,
}
impl ColorPalette {
    pub fn new() -> Self {
        Self {
            colors: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorPaletteIndex(u16);
