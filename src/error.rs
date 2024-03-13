use thiserror::Error;

#[repr(u8)]
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


impl RuptureError {
    pub(crate) fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}