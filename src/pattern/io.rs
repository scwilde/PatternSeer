use std::{
    fmt::Display,
    fs::File,
    io::{
        self,
        Write,
    }, path::Path
};

use super::{Pattern, stitch_buffer::StitchBuffer};


pub trait BinaryChunk {
    fn to_le_bytes(&self) -> Box<[u8]>;
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FourCC([u8; 4]);
impl FourCC {
    pub fn from_le_bytes(bytes: &[u8; 4]) -> Self {
        Self(*bytes)
    }

    pub fn to_le_bytes(self) -> [u8; 4] {
        self.0
    }

    pub fn as_le_bytes(&self) -> &[u8; 4] {
        &self.0
    }
}
impl Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[derive(Debug)]
pub enum PspatError {
    BadMagic,
    MissingRequiredChunks(Box<[FourCC]>),
    OsReadError(io::Error),
    OsWriteError(io::Error),
    IncompleteChunk,
}
impl Display for PspatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadMagic => write!(f, "this is not a PatternSeer pattern"),
            Self::MissingRequiredChunks(chunks) => {
                let chunks = chunks.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                        .join(",");
                write!(f, "file is missing the required chunks: {}", chunks)
            },
            Self::OsReadError(e) => write!(f, "there was an OS-level error while reading the file: {}", e),
            Self::OsWriteError(e) => write!(f, "there was an OS-level error while writing the file: {}", e),
            Self::IncompleteChunk => write!(f, "found EOF before all of the chunk's stated bytes were loaded"),
        }
    }
}
impl PartialEq for PspatError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BadMagic, Self::BadMagic) => true,
            (Self::MissingRequiredChunks(a), Self::MissingRequiredChunks(b)) => a == b,
            (Self::OsReadError(a), Self::OsReadError(b)) => a.kind() == b.kind(),
            (Self::OsWriteError(a), Self::OsWriteError(b)) => a.kind() == b.kind(),
            _ => false
        }
    }
}

struct FileChunkIter<R: io::Read> {
    reader: R,
}
impl<R: io::Read> FileChunkIter<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}
impl<R: io::Read> Iterator for FileChunkIter<R> {
    type Item = Result<OwnedFileChunk, PspatError>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_chunk = read_chunk(&mut self.reader);

        if next_chunk.is_ok() {
            Some(next_chunk)
        } else {
            match next_chunk.as_ref().unwrap_err() {
                PspatError::IncompleteChunk => None,
                _ => Some(next_chunk)
            }
        }
    }
}

#[derive(Debug)]
enum BorrowedFileChunk<'a> {
    RAST(&'a StitchBuffer),
    XTRA { original_type: &'a FourCC, original_bytes: &'a [u8] },
}
#[derive(Debug)]
enum OwnedFileChunk  {
    RAST(StitchBuffer),
    XTRA { original_type: FourCC, original_bytes: Box<[u8]> },
}

struct PatternLoader {
    width: u16,
    height: u16,
    raster_layer: Option<StitchBuffer>,
    // stitch_palette: Option<StitchPalette>,
    // color_palette: Option<ColorPalette>,
    // metadata
    // vetcor_layer
}
impl PatternLoader {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            raster_layer: None,
        }
    }
    pub fn set_raster_layer(&mut self, raster_layer: StitchBuffer) {
        self.raster_layer = Some(raster_layer);
    }
    pub fn validate(&self) -> Result<(), Vec<FourCC>> {
        let mut missing_chunks = vec![];

        if self.raster_layer.is_none() { 
            missing_chunks.push(FourCC::from_le_bytes(b"PRIM"));
        }

        if missing_chunks.len() > 0 {
            Err(missing_chunks)
        } else {
            Ok(())
        }
    }
    pub fn build(self) -> Result<Pattern, PspatError> {
        match self.validate() {
            Ok(_) => {
                Ok(Pattern {
                    width: self.width,
                    height: self.height,
                    primary_grid: self.raster_layer.unwrap(),
                })
            },
            Err(missing_chunks) => Err(PspatError::MissingRequiredChunks(missing_chunks.into())),
        }
    }
}


/// Writes the header section of a pattern file.
/// 
/// # Parameters
/// 
/// - `writer`: File writer to write the header to.
/// - `width`: Width of the pattern.
/// - `height`: Height of the pattern.
/// 
/// # Returns
/// 
/// `io::Result` which can be either:
/// - `Ok(())` when all is well.
/// - `Err(io::Error)` If something went wrong while writing the header.
fn write_header<W: io::Write>(writer: &mut W, width: u16, height: u16) -> Result<(), PspatError> {
    writer.write_all(b"\x00PsPat")
        .map_err(|e| PspatError::OsWriteError(e))?;
    writer.write_all(&[1u8, 0u8])
        .map_err(|e| PspatError::OsWriteError(e))?;
    writer.write_all(&width.to_le_bytes())
        .map_err(|e| PspatError::OsWriteError(e))?;
    writer.write_all(&height.to_le_bytes())
        .map_err(|e| PspatError::OsWriteError(e))?;
    Ok(())
}

fn write_chunk<W: io::Write>(writer: &mut W, chunk: BorrowedFileChunk) -> Result<(), PspatError> {
    let bytes = match chunk {
        BorrowedFileChunk::RAST(stitches) => {
            writer.write_all(b"RAST")
                .map_err(|e| PspatError::OsWriteError(e))?;
            stitches.to_le_bytes()
        },
        BorrowedFileChunk::XTRA { original_type, original_bytes } => {
            writer.write_all(original_type.as_le_bytes())
                .map_err(|e| PspatError::OsWriteError(e))?;
            original_bytes.into()
        }
    };
    let num_bytes = u32::try_from(bytes.len()).unwrap();

    writer.write_all(&num_bytes.to_le_bytes())
        .map_err(|e| PspatError::OsWriteError(e))?;
    writer.write_all(&bytes)
        .map_err(|e| PspatError::OsWriteError(e))?;

    Ok(())
}

/// Saves a pattern to disk.
/// 
/// # Parameters
/// 
/// - `path`: Pattern save location.
/// - `pattern`: Pattern to be saved.
/// 
/// # Returns
/// 
/// `io::Result` which can be either:
/// - `Ok(())` when all is well.
/// - `Err(io::Error)` If something went wrong while writing the file.
pub fn save(path: &Path, pattern: &Pattern) -> Result<(), PspatError> {
    let file = File::create(path)
        .map_err(|e| PspatError::OsWriteError(e))?;
    let mut writer = io::BufWriter::new(file);

    write_header(&mut writer, pattern.width, pattern.height)?;
    write_chunk(&mut writer, BorrowedFileChunk::RAST(&pattern.primary_grid))?;

    writer.flush()
        .map_err(|e| PspatError::OsWriteError(e))?;
    Ok(())
}


/// Reads the header of a pattern file.
/// 
/// # Parameters
/// 
/// - `reader`: File reader to read the pattern from
///
/// # Returns
/// 
/// `io::Result` which can be either:
/// - `Ok((width, height))` when all is well.
/// - `Err(io::Error)` If something went wrong while reading the header.
fn read_header<R: io::Read>(reader: &mut R) -> Result<(u16, u16), PspatError> {
    let mut magic = [0u8; 6];
    reader.read_exact(&mut magic)
        .map_err(|e| PspatError::OsReadError(e))?;
    //TODO Magic value. Make into a global constant.
    if &magic != b"\x00PsPat" {
        return Err(PspatError::BadMagic);
    }

    let mut version = [0u8; 2];
    reader.read_exact(&mut version)
        .map_err(|e| PspatError::OsReadError(e))?;

    let mut width_bytes = [0u8; 2];
    let mut height_bytes = [0u8; 2];
    reader.read_exact(&mut width_bytes)
        .map_err(|e| PspatError::OsReadError(e))?;
    reader.read_exact(&mut height_bytes)
        .map_err(|e| PspatError::OsReadError(e))?;

    Ok((u16::from_le_bytes(width_bytes), u16::from_le_bytes(height_bytes)))
}

fn read_chunk<R: io::Read>(reader: &mut R) -> Result<OwnedFileChunk, PspatError> {
    let mut fourcc = [0u8; 4];
    reader.read_exact(&mut fourcc)
        .map_err(|e| PspatError::OsReadError(e))?;
    let fourcc = FourCC::from_le_bytes(&fourcc);

    let mut chunk_len = [0u8; 4];
    reader.read_exact(&mut chunk_len)
        .map_err(|e| PspatError::OsReadError(e))?;
    let chunk_len = u32::from_le_bytes(chunk_len);

    let mut chunk_bytes = vec![0u8; chunk_len as usize];
    reader.read_exact(&mut chunk_bytes).map_err(|e| {
        match e.kind() {
            io::ErrorKind::UnexpectedEof => PspatError::IncompleteChunk,
            _ => PspatError::OsReadError(e),
        }
    })?;

    // TODO Check the fourcc first so that an unrecognized chunks that dont need to be stored as XTRA  are not even read
    Ok(match fourcc.as_le_bytes() {
        b"RAST" => { OwnedFileChunk::RAST(StitchBuffer::from_le_bytes(&chunk_bytes)) },
        _ => { OwnedFileChunk::XTRA { original_type: fourcc, original_bytes: chunk_bytes.into() } },
    })
}

/// Loads a pattern from disk.
/// 
/// # Parameters
/// 
/// - `path`: Pattern location.
/// 
/// # Returns
/// 
/// `io::Result` which can be either:
/// - `Ok(Pattern)` when all is well.
/// - `Err(io::Error)` If something went wrong while reading the file.
pub fn load(path: &Path) -> Result<Pattern, PspatError> {
    let file = File::open(&path)
        .map_err(|e| PspatError::OsReadError(e))?;
    let mut reader = io::BufReader::new(file);

    let (width, height) = read_header(&mut reader)?;
    let mut loaded_pattern = PatternLoader::new(width, height);

    for chunk in FileChunkIter::new(reader) {
        match chunk? {
            OwnedFileChunk::RAST(raster_layer) => loaded_pattern.set_raster_layer(raster_layer),
            OwnedFileChunk::XTRA { .. } => {},
        }
    }
    loaded_pattern.build()
}



#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::ops::Range;
    
    use super::*;

    mod header_layout {
        const MAGIC_OFFSET: usize = 0;
        const MAGIC_STRIDE: usize = 6;
        pub const MAGIC: super::Range<usize> = MAGIC_OFFSET..(MAGIC_OFFSET + MAGIC_STRIDE);

        const VERSION_OFFSET: usize = 6;
        const VERSION_STRIDE: usize = 2;
        pub const VERSION: super::Range<usize> = VERSION_OFFSET..(VERSION_OFFSET + VERSION_STRIDE);

        const WIDTH_OFFSET: usize = 8;
        const WIDTH_STRIDE: usize = 2;
        pub const WIDTH: super::Range<usize> = WIDTH_OFFSET..(WIDTH_OFFSET + WIDTH_STRIDE);
        
        const HEIGHT_OFFSET: usize = 10;
        const HEIGHT_STRIDE: usize = 2;
        pub const HEIGHT: super::Range<usize> = HEIGHT_OFFSET..(HEIGHT_OFFSET + HEIGHT_STRIDE);
    }
    #[test]
    fn test_write_header_magic() {
        let expected_magic = b"\x00PsPat";

        let mut buf = vec![];
        write_header(&mut buf, 0, 0).unwrap();

        assert_eq!(&buf[header_layout::MAGIC], expected_magic);
    }
    #[test]
    fn test_write_header_version() {
        let expected_version = [1u8, 0u8];

        let mut buf = vec![];
        write_header(&mut buf, 0, 0).unwrap();

        assert_eq!(&buf[header_layout::VERSION], expected_version);
    }
    #[test]
    fn test_write_header_dimensions_square() {
        let (width, height) = (30, 30);


        let mut buf = vec![];
        write_header(&mut buf, width, height).unwrap();

        assert_eq!(buf[header_layout::WIDTH], width.to_le_bytes());
        assert_eq!(buf[header_layout::HEIGHT], height.to_le_bytes());
    }
    // TODO test writing non-square headers
    #[test]
    fn test_read_header_bad_magic() {
        let garbage = b"\x00BadMagichsdgjashdjhafhgasjhfjhsajhgfjh";

        let mut cursor = Cursor::new(garbage);
        let result = read_header(&mut cursor);

    Ok(Pattern::new(width, height, primary_grid.unwrap()))
}
