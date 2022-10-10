use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum ArchiveError {
    HEDReadError,
    HEDCreateError,
    HEDWriteError,
    HEDFormatError,
    DATReadError,
    DATCreateError,
    DATWriteError,
    NamesFormatError,
    OffsetError,
    LengthError,
    UnpackError,
    PackError,
    FileNotPresentError,
    SerializeError,
    FileStateError,
    TempFileCreateError,
}

impl Display for ArchiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ArchiveError {}
