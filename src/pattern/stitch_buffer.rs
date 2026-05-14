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

    pub fn replace(&mut self, offset: usize, new_vals: &[u16]) -> Result<(), String> {
        if new_vals.len() + offset > self.len() {
            return Err(String::from("replacement array will overflow the stitch buffer"));
        }

        // TODO this is inefficient
        for (index, new_val) in new_vals.iter().enumerate() {
            self.0.get_mut(index + offset).unwrap().swap(*new_val);
        }

        Ok(())
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



// ! The following iterator, while useful, requires the use of an unsafe block 
// pub struct StitchBuffer2dSlice<'a> {
//     source: &'a mut StitchBuffer,

//     width: u16,
//     height: u16,
//     stride: u16,

//     index: usize,
// }
// impl<'a> StitchBuffer2dSlice<'a> {
//     pub fn new(source: &'a mut StitchBuffer, slice: utils::Bounds2d<u16>, stride: u16) -> Result<Self, String> {
//         let start_index = slice.x.min + (slice.y.min * stride);
//         let end_index = slice.x.max + (slice.y.max * stride);
//         if let None = source.get_mut(start_index as usize) {
//             return Err(String::from("slice start point if after the end of the source buffer"));
//         }
//         if let None = source.get_mut(end_index as usize) {
//             return Err(String::from("slice overflows the bounds of the source buffer"));
//         }

//         Ok(StitchBuffer2dSlice {
//             source,
//             width: slice.x.max - slice.x.min,
//             height: slice.y.max - slice.y.min,
//             stride, 
//             index: start_index as usize
//         })
//     }
// }
// impl<'a> Iterator for StitchBuffer2dSlice<'a> {
//     type Item = &'a mut StitchPaletteIndex;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.source.get_mut(self.index)
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
}
