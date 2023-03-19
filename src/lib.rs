use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub mod toml;

fn config_path<P>(default: P) -> PathBuf
where
    P: AsRef<Path>,
{
    if let Some(path) = std::env::args().nth(1) {
        return PathBuf::from(path);
    }

    default.as_ref().to_path_buf()
}

fn config_string<P>(config_path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut config_file = File::open(config_path)?;
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string)?;
    Ok(config_string)
}
