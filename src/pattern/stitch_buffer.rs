use super::{
    stitch_palette::StitchPaletteIndex,
    io::BinaryChunk,
};

#[derive(Debug, PartialEq)]
pub struct StitchBuffer(Vec<StitchPaletteIndex>);
impl StitchBuffer {
    pub fn with_size(width: u16, height: u16) -> Self {
        Self(vec![StitchPaletteIndex(0); (width * height) as usize])
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut StitchPaletteIndex> {
        self.0.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl BinaryChunk for StitchBuffer {
    fn to_le_bytes(&self) -> Box<[u8]> {
        let mut bytes = vec![];
        for stitch in &self.0 {
            bytes.extend(stitch.to_le_bytes());
        }
        bytes.into()
    }

    fn from_le_bytes(bytes: &[u8]) -> Self {
        let mut stitches = vec![];
        for pair in bytes.chunks_exact(2) {
            stitches.push(StitchPaletteIndex::from_le_bytes(pair.try_into().unwrap()));
        }
        Self(stitches)
    }
}
