use std::ffi::OsString;

use crate::settings::RuptureSettings;

struct Rupture {
    source_name: OsString,
    settings: RuptureSettings,
}


struct RuptureBlock(Vec<u8>);