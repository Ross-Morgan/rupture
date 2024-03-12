#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod juncture;
pub mod rupture;

pub type Result<T> = std::result::Result<T, error::RuptureError>;
