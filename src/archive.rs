use crate::*;
use std::{
    convert::TryInto,
    io::Seek,
    io::{Read, SeekFrom, Write},
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

    fn write_dat_block<P>(dat_path: P, data: &[u8], packed: bool) -> Result<HEDEntry, ArchiveError>
    where
        P: AsRef<Path>,
    {
        let unpacked_len = data.len();
        let packed_data = if packed {
            compression::pack(data).map_err(|_| ArchiveError::PackError)?
        } else {
            data.to_vec()
        };
        let packed_len = packed_data.len();

        let mut out_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(dat_path)
            .map_err(|_| ArchiveError::DATWriteError)?;

        let start = out_file
            .seek(SeekFrom::End(0))
            .map_err(|_| ArchiveError::HEDWriteError)?;
        out_file
            .write_all(&packed_data)
            .map_err(|_| ArchiveError::HEDWriteError)?;

        Ok(HEDEntry {
            packed,
            start: start.try_into().map_err(|_| ArchiveError::OffsetError)?,
            packed_len: packed_len
                .try_into()
                .map_err(|_| ArchiveError::LengthError)?,
            unpacked_len: unpacked_len
                .try_into()
                .map_err(|_| ArchiveError::LengthError)?,
        })
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

    pub fn remove_file(&mut self, file_name: &str) {
        let mut indices_to_remove = Vec::<usize>::new();

        // Find the index or indices with this file name
        for (i, name) in self.file_names.iter().enumerate() {
            if name == file_name {
                indices_to_remove.push(i);
            }
        }

        // Remove the file name and its entry
        for i in indices_to_remove {
            self.file_names.remove(i);
            self.hed.remove_entry(i + 1); // 0 is reserved for file list
        }
    }

    pub fn add_file(&mut self, file_name: &str, data: &[u8]) -> Result<(), ArchiveError> {
        // Make sure no duplicate files are made
        self.remove_file(file_name);

        // These should always correspond to each other, with there being 1 more
        // entry for the list of file names saved to disk.
        if self.hed.entries.len() != self.file_names.len() + 1 {
            println!("{} {}", self.hed.entries.len(), self.file_names.len());
            return Err(ArchiveError::FileStateError);
        }

        // Write file contents to DAT
        let file_entry = Self::write_dat_block(&self.dat_path, data, true)?;

        self.hed.entries.push(file_entry);
        self.file_names.push(file_name.to_string());

        Ok(())
    }

    pub fn finalize(&mut self) -> Result<(), ArchiveError> {
        // Add new file name list
        let serialized_names = dat::serialize_file_names(&self.file_names)?;
        let file_list_entry = Self::write_dat_block(&self.dat_path, &serialized_names, true)?;

        // Update HED
        self.hed.set_files_entry(&file_list_entry);

        // Write HED
        self.hed.write(&self.hed_path)?;

        Ok(())
    }
}
