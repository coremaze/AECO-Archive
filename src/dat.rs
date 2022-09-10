use crate::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Write};

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

pub fn serialize_file_names(file_names: &[String]) -> Result<Vec<u8>, ArchiveError> {
    let count = if file_names.len() > u32::MAX as usize {
        return Err(ArchiveError::LengthError);
    } else {
        file_names.len() as u32
    };

    let mut vec = Vec::<u8>::new();

    // Length prefix the string array
    vec.write_u32::<LittleEndian>(count)
        .map_err(|_| ArchiveError::DATWriteError)?;

    for name in file_names {
        if !name.is_ascii() {
            return Err(ArchiveError::NamesFormatError);
        }
        let name_bytes = name.as_bytes();
        // Write the file name
        vec.write(name_bytes)
            .map_err(|_| ArchiveError::DATWriteError)?;
        // Write a null terminator
        vec.write_u8(0).map_err(|_| ArchiveError::DATWriteError)?;
    }

    // Extra null byte
    vec.write_u8(0).map_err(|_| ArchiveError::DATWriteError)?;

    Ok(vec)
}
