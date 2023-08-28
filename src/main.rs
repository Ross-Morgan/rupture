use std::{ffi::OsString, path::PathBuf};

use rupture::{const_split_file, RuptureError};

fn main() -> Result<(), RuptureError> {
    const_split_file::<512>(PathBuf::from("./large-file.txt"), OsString::from("file"))?;

    Ok(())
}
