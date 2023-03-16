use std::{
    fs::File,
    io::{self, Read},
};

pub mod toml;

fn config_path(default: &str) -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| default.to_string())
}

fn config_string(default_path: &str) -> io::Result<String> {
    let config_path = config_path(default_path);
    let mut config_file = File::open(config_path)?;
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;
    Ok(config_string)
}
