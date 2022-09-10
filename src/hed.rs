use crate::ArchiveError;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::path::Path;

#[derive(Debug)]
pub struct HED {
    pub entries: Vec<HEDEntry>,
}

#[derive(Debug)]
pub struct HEDEntry {
    pub packed: bool,
    // Normally file offsets should be u64, but only u32s are supported by the format.
    pub start: u32,
    pub packed_len: u32,
    pub unpacked_len: u32,
}

impl HED {
    pub fn new<P>(path: P) -> Result<Self, ArchiveError>
    where
        P: AsRef<Path>,
    {
        // Create file if it doesn't exist yet
        if !path.as_ref().exists() {
            std::fs::File::create(&path).or_else(|_| Err(ArchiveError::ReadError))?;
        }

        // Read existing file
        let data = std::fs::read(&path).or_else(|_| Err(ArchiveError::ReadError))?;
        let data_len = data.len() as u64;

        let mut cursor = Cursor::new(data);
        let mut entries = Vec::<HEDEntry>::with_capacity(3 * (u32::BITS / u8::BITS) as usize);

        while cursor.position() < data_len {
            let entry = HED::try_deserialize_entry(&mut cursor)
                .or_else(|_| Err(ArchiveError::FormatError))?;
            entries.push(entry);
        }

        Ok(Self { entries })
    }

    fn try_deserialize_entry(cursor: &mut std::io::Cursor<Vec<u8>>) -> std::io::Result<HEDEntry> {
        let start = cursor.read_u32::<LittleEndian>()?;
        let packed_field = cursor.read_u32::<LittleEndian>()?;
        let packed: bool = packed_field & 0x80000000 != 0;
        let packed_len: u32 = packed_field & 0x7FFFFFFF;
        let unpacked_len = cursor.read_u32::<LittleEndian>()?;

        Ok(HEDEntry {
            packed,
            start,
            packed_len,
            unpacked_len,
        })
    }
}
