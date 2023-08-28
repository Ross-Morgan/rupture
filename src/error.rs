use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuptureError {
    #[error("Failed to open file '{0:?}'")]
    OpenFileError(std::io::Error),
    #[error("Failed to write to file '{0:?}'")]
    WriteFileError(std::io::Error),
    #[error("Failed to create file called '{0:?}'")]
    CreateFileError(std::io::Error),
    #[error("Failed to create dir called '{0:?}'")]
    CreateDirError(std::io::Error),
}
