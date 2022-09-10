use crate::ArchiveError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};
use std::path::Path;

#[derive(Debug)]
pub struct HED {
    pub entries: Vec<HEDEntry>,
}

#[derive(Debug, Clone, Copy)]
pub struct HEDEntry {
    pub packed: bool,
    // Normally file offsets should be u64, but only u32s are supported by the format.
    pub start: u32,
    pub packed_len: u32,
    pub unpacked_len: u32,
}

impl HEDEntry {
    pub fn is_end_marker(&self) -> bool {
        !self.packed && self.start == 0 && self.packed_len == 0 && self.unpacked_len == 0
    }

    pub fn end_marker() -> Self {
        Self {
            packed: false,
            start: 0,
            packed_len: 0,
            unpacked_len: 0,
        }
    }
}

impl HED {
    pub fn new<P>(path: P) -> Result<Self, ArchiveError>
    where
        P: AsRef<Path>,
    {
        // Create file if it doesn't exist yet
        if !path.as_ref().exists() {
            std::fs::File::create(&path).map_err(|_| ArchiveError::HEDReadError)?;
        }

        // Read existing file
        let data = std::fs::read(&path).map_err(|_| ArchiveError::HEDReadError)?;
        let data_len = data.len() as u64;

        let mut cursor = Cursor::new(data);
        let mut entries = Vec::<HEDEntry>::with_capacity((data_len / 12) as usize);

        loop {
            let entry = HED::try_deserialize_entry(&mut cursor)
                .map_err(|_| ArchiveError::HEDFormatError)?;

            // The end is signified with an empty entry
            if entry.is_end_marker() {
                break;
            }

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

    fn serialize(&self) -> Result<Vec<u8>, ArchiveError> {
        let mut vec = Vec::<u8>::with_capacity(self.entries.len() * 3 * 4);

        for entry in &self.entries {
            // A packed length over the max i32 cannot be expressed
            if entry.packed_len & 0x80000000 != 0 {
                return Err(ArchiveError::LengthError);
            }

            Self::try_serialize_entry(&mut vec, entry).map_err(|_| ArchiveError::SerializeError)?;
        }

        // The end is signified with an empty entry
        Self::try_serialize_entry(&mut vec, &HEDEntry::end_marker())
            .map_err(|_| ArchiveError::SerializeError)?;

        Ok(vec)
    }

    fn try_serialize_entry(vec: &mut Vec<u8>, entry: &HEDEntry) -> std::io::Result<()> {
        let start = entry.start;

        // The top bit of the packed field represents whether the data is compressed
        let packed_field = if entry.packed {
            entry.packed_len | 0x80000000
        } else {
            entry.packed_len
        };

        let unpacked_len = entry.unpacked_len;

        vec.write_u32::<LittleEndian>(start)?;
        vec.write_u32::<LittleEndian>(packed_field)?;
        vec.write_u32::<LittleEndian>(unpacked_len)?;

        Ok(())
    }

    pub fn write<P>(&self, path: P) -> Result<(), ArchiveError>
    where
        P: AsRef<Path>,
    {
        let serialized_data = self.serialize()?;
        let mut out_file = std::fs::File::create(path).map_err(|_| ArchiveError::HEDWriteError)?;

        out_file
            .write_all(&serialized_data)
            .map_err(|_| ArchiveError::HEDWriteError)?;

        Ok(())
    }

    pub fn remove_entry(&mut self, index: usize) {
        if index < self.entries.len() {
            self.entries.remove(index);
        }
    }

    pub fn set_files_entry(&mut self, hed_entry: &HEDEntry) {
        if let Some(entry) = self.entries.get_mut(0) {
            *entry = *hed_entry;
        } else {
            self.entries.push(*hed_entry);
        }
    }
}
