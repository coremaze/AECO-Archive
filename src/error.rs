#[derive(Debug)]
pub enum ArchiveError {
    ReadError,
    CreateError,
    WriteError,
    FormatError,
}
