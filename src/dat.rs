use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

pub fn deserialize_file_names(data: &[u8]) -> Result<Vec<String>, ArchiveError> {
    let mut cursor = Cursor::new(data);

    try_deserialize_file_names(&mut cursor).map_err(|_| ArchiveError::NamesFormatError)
}

fn try_deserialize_file_names(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Vec<String>> {
    let name_count = cursor.read_u32::<LittleEndian>()? as usize;

    let mut file_names = Vec::with_capacity(name_count);

    let mut file_name = String::new();
    for _ in 0..name_count {
        loop {
            let character = cursor.read_u8()?;
            if character == 0 {
                file_names.push(file_name);
                file_name = String::new();
                break;
            } else {
                file_name.push(character as char);
            }
        }
    }

    // There should also be an extra null byte at the end

    Ok(file_names)
}
