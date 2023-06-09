use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

use crate::config_string;

const DEFAULT_CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("TOML error")]
    Toml(#[from] toml::de::Error),
}

pub fn get_config<C>() -> Result<C, ConfigError>
where
    C: for<'de> Deserialize<'de> + std::fmt::Debug,
{
    get_config_from_path(config_path())
}

pub fn config_path() -> PathBuf {
    crate::config_path(DEFAULT_CONFIG_PATH)
}

pub fn get_config_from_path<C, P>(path: P) -> Result<C, ConfigError>
where
    C: for<'de> Deserialize<'de> + std::fmt::Debug,
    P: AsRef<Path>,
{
    let config_string = config_string(path)?;
    let config = toml::from_str(&config_string)?;

    Ok(config)
}
