//! CrabbyAuth 🦀
//! 
//! A comprehensive OAuth and SCIM implementation in Rust.
//! We take security seriously, but we do it with a crabby attitude!

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod oauth;
pub mod scim;
pub mod storage;
pub mod plugin;
pub mod security;
pub mod monitoring;
pub mod i18n;
pub mod enterprise;
pub mod utils;
pub mod error;
pub mod config;

pub use error::Error;
pub use config::Config;

/// Library version - with extra crab attitude
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CRABBY_MASCOT: &str = "🦀";
