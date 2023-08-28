use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

pub fn file_name_without_extension(file: PathBuf, default: &OsStr) -> OsString {
    file.file_stem().unwrap_or(default).to_os_string()
}

macro_rules! concat_os {
    ($($os_string:expr),+ $(,)?) => {
        {
            let cap = 0usize $( + $os_string.len() )+;

            let mut res = std::ffi::OsString::with_capacity(cap);

            $( res.push($os_string.as_os_str()); )+

            res
        }
    };
}

pub(crate) use concat_os;
