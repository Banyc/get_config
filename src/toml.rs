use std::{fs::File, io::Read};

use serde::Deserialize;
use thiserror::Error;

use crate::config_path;

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
    let config_path = config_path(DEFAULT_CONFIG_PATH);
    let mut config_file = File::open(config_path)?;
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;
    let config = toml::from_str(&config)?;

    Ok(config)
}
