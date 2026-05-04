use std::{
    fs::File,
    io::{
        self,
        Write,
    },
    path::{Path, PathBuf},
};
use crate::pattern::Pattern;


enum ChunkType {
    
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

fn write_stitch_palette() {}
fn write_color_palette() {}
fn write_primary_grid() {}

fn write_chunk() {}

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
pub fn load(path: PathBuf) -> io::Result<Pattern> {
    let file = File::open(&path)?;
    let mut reader = io::BufReader::new(file);

    let (width, height) = read_header(&mut reader)?;

    Ok(Pattern::from_file(width, height, path))
}
