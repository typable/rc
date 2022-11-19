mod config;
mod error;
mod log;

pub mod prelude {
    pub use crate::config::*;
    pub use crate::error::*;
    pub use crate::log::LogLevel;
    pub use crate::*;
}

pub const APP_NAME: &str = "rc";
pub const CONFIG_FILE: &str = "config.toml";
