
use std::{fs::File, io::{self, BufReader, BufWriter, Write}, path::Path};
use crate::pattern::Pattern;


//? Need this to create a new pattern? Or does save() add all the binary structure on each save?
// pub fn create() {}

fn write_header<W: io::Write>(writer: &mut W, width: u16, height: u16) -> io::Result<()> {
    writer.write_all(&[0])?;
    writer.write_all(b"PsPat")?;
    writer.write_all(&[1, 0])?;
    writer.write_all(&width.to_le_bytes())?;
    writer.write_all(&height.to_le_bytes())?;
    Ok(())
}

pub fn save(path: &Path, pattern: &Pattern) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    write_header(&mut writer, pattern.width, pattern.height)?;
    
    writer.flush()?;
    Ok(())
}

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

pub fn load(path: &Path) -> io::Result<Pattern> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    let (width, height) = read_header(&mut reader)?;
    
    Ok(Pattern { width, height })
}
