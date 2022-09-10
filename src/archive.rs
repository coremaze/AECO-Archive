use crate::*;
use std::{
    io::Seek,
    io::{Read, SeekFrom},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Archive {
    dat_path: PathBuf,
    hed_path: PathBuf,
    hed: HED,
    file_names: Vec<String>,
}

impl Archive {
    pub fn open_pair<P>(dat_path: P, hed_path: P) -> Result<Self, ArchiveError>
    where
        P: AsRef<Path>,
    {
        let hed = HED::new(&hed_path)?;

        // Names are always the first element in HED entries
        let names_entry = hed.entries.get(0).ok_or(ArchiveError::HEDFormatError)?;
        let names_serialized = Self::read_dat_block(&dat_path, names_entry)?;
        let file_names = dat::deserialize_file_names(&names_serialized)?;

        Ok(Self {
            dat_path: dat_path.as_ref().to_path_buf(),
            hed_path: hed_path.as_ref().to_path_buf(),
            hed,
            file_names,
        })
    }

    fn read_dat_block<P>(dat_path: P, hed_entry: &HEDEntry) -> Result<Vec<u8>, ArchiveError>
    where
        P: AsRef<Path>,
    {
        let mut dat_file = std::fs::File::open(dat_path).map_err(|_| ArchiveError::DATReadError)?;
        // Start reading from the start of the block as referenced by the hed entry
        dat_file
            .seek(SeekFrom::Start(hed_entry.start as u64))
            .map_err(|_| ArchiveError::OffsetError)?;

        let mut potentially_packed_buf = vec![0u8; hed_entry.packed_len as usize];
        match dat_file.read(&mut potentially_packed_buf) {
            Ok(amount) => {
                // Make sure the correct number of bytes were read
                if amount != hed_entry.packed_len as usize {
                    return Err(ArchiveError::LengthError);
                }
            }
            Err(_) => {
                return Err(ArchiveError::DATReadError);
            }
        }

        let result_buf = if hed_entry.packed {
            compression::unpack_sized(&potentially_packed_buf, hed_entry.unpacked_len as usize)
                .map_err(|_| ArchiveError::UnpackError)?
        } else {
            potentially_packed_buf
        };

        Ok(result_buf)
    }

    pub fn file_names(&self) -> &[String] {
        &self.file_names
    }

    pub fn get_file(&self, file_name: &str) -> Result<Vec<u8>, ArchiveError> {
        let mut hed_file_index: Option<usize> = None;

        for (i, f) in self.file_names.iter().enumerate() {
            // Need to skip the first HED index because that refers to the file name list
            let hed_index = i + 1;

            if file_name == f {
                hed_file_index = Some(hed_index);
                break;
            }
        }

        let hed_file_index = match hed_file_index {
            Some(x) => x,
            None => {
                return Err(ArchiveError::FileNotPresentError);
            }
        };

        let hed_entry = self
            .hed
            .entries
            .get(hed_file_index)
            .ok_or(ArchiveError::HEDFormatError)?;

        Self::read_dat_block(&self.dat_path, hed_entry)
    }
}
