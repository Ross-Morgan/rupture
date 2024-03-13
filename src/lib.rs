#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod juncture;
pub mod rupture;

pub type Result<T> = std::result::Result<T, error::RuptureError>;

pub const RUPTURE_EXTENSION: &'static str = "rupt";

/// Helper function to ensure all number-to-bytes conversions follow the same byte ordering
const fn num_to_bytes(n: u64) -> [u8; 8] {
    n.to_be_bytes()
}

/// Helper function to ensure all bytes-to-number conversions follow the same byte ordering
const fn bytes_to_num(bytes: [u8; 8]) -> u64 {
    u64::from_be_bytes(bytes)
}

pub mod prelude {
    use super::*;

    pub use super::RUPTURE_EXTENSION;

    pub use error::RuptureError;
    pub use juncture::juncture_file;
    pub use rupture::rupture_file;
} 