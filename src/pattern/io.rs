use std::{
    fs::File,
    io::{
        self,
        Write,
    },
    path::{Path, PathBuf},
};
use super::{Pattern, stitch_buffer::StitchBuffer};

pub trait BinaryChunk {
    fn to_le_bytes(&self) -> Box<[u8]>;
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

#[derive(Clone)]
struct FourCC([u8; 4]);
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
impl PartialEq<[u8]> for FourCC {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == *other
    }
}

enum RefChunk<'a> {
    PRIM(&'a StitchBuffer),
    XTRA { original_type: &'a FourCC, original_bytes: &'a [u8] },
}

enum OwnedChunk  {
    PRIM(StitchBuffer),
    XTRA { original_type: FourCC, original_bytes: Box<[u8]> },
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
fn write_header<W: io::Write>(writer: &mut W, width: u16, height: u16) -> io::Result<()> {
    writer.write_all(&[0])?;
    writer.write_all(b"PsPat")?;
    writer.write_all(&[1, 0])?;
    writer.write_all(&width.to_le_bytes())?;
    writer.write_all(&height.to_le_bytes())?;
    Ok(())
}

fn write_chunk<W: io::Write>(writer: &mut W, chunk: RefChunk) -> io::Result<()> {
    let bytes = match chunk {
        RefChunk::PRIM(stitches) => {
            writer.write_all(b"PRIM")?;
            stitches.to_le_bytes()
        },
        RefChunk::XTRA { original_type, original_bytes } => {
            writer.write_all(original_type.as_le_bytes())?;
            original_bytes.into()
        }
    };
    let num_bytes = u32::try_from(bytes.len()).unwrap();

    writer.write_all(&num_bytes.to_le_bytes())?;
    writer.write_all(&bytes)?;

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
pub fn save(path: &Path, pattern: &Pattern) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = io::BufWriter::new(file);

    write_header(&mut writer, pattern.width, pattern.height)?;
    write_chunk(&mut writer, RefChunk::PRIM(&pattern.primary_grid))?;

    writer.flush()?;
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
fn read_header<R: io::Read>(reader: &mut R) -> io::Result<(u16, u16)> {
    let mut magic = [0u8; 6];
    reader.read_exact(&mut magic)?;
    if &magic != b"\x00PsPat" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid PatternSeer pattern"));
    }

    let mut version = [0u8; 2];
    reader.read_exact(&mut version)?;

    let mut width_bytes = [0u8; 2];
    let mut height_bytes = [0u8; 2];
    reader.read_exact(&mut width_bytes)?;
    reader.read_exact(&mut height_bytes)?;

    Ok((u16::from_le_bytes(width_bytes), u16::from_le_bytes(height_bytes)))
}

fn read_chunk<R: io::Read>(reader: &mut R) -> io::Result<OwnedChunk> {
    let mut fourcc = [0u8; 4];
    reader.read_exact(&mut fourcc)?;
    let fourcc = FourCC::from_le_bytes(&fourcc);

    let mut chunk_len = [0u8; 4];
    reader.read_exact(&mut chunk_len)?;
    let chunk_len = u32::from_le_bytes(chunk_len);

    let mut bytes = vec![0u8; chunk_len as usize];
    reader.read_exact(&mut bytes)?;

    Ok(match fourcc.as_le_bytes() {
        b"PRIM" => { OwnedChunk::PRIM(StitchBuffer::from_le_bytes(&bytes)) },
        _ => { OwnedChunk::XTRA { original_type: fourcc, original_bytes: bytes.into() } },
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
pub fn load(path: &Path) -> io::Result<Pattern> {
    let file = File::open(&path)?;
    let mut reader = io::BufReader::new(file);

    let (width, height) = read_header(&mut reader)?;
    let mut primary_grid: Option<StitchBuffer> = None;
    match read_chunk(&mut reader)? {
        OwnedChunk::PRIM(buffer) => primary_grid = Some(buffer),
        OwnedChunk::XTRA { .. } => todo!(),
    }

    Ok(Pattern::new(width, height, primary_grid.unwrap()))
}
