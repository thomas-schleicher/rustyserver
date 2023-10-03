use std::collections::HashMap;
use std::io::ErrorKind;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    address: Option<String>,
    port: Option<u16>,
    pages: Option<HashMap<String, String>>
}

pub fn load(filepath: &str) -> Result<Configuration, ErrorKind> {

    let data_string = match std::fs::read_to_string(filepath) {
        Ok(file) => file,
        Err(error) => return Err(error.kind())
    };

    let config: Configuration = match toml::from_str(&data_string) {
        Ok(config) => config,
        Err(_) => return Err(ErrorKind::Other)
    };

    Ok(config)
}

// pub fn create() {}