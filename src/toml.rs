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
    let config_string = config_string(&config_path())?;
    let config = toml::from_str(&config_string)?;

    Ok(config)
}

pub fn config_path() -> String {
    crate::config_path(DEFAULT_CONFIG_PATH)
}
