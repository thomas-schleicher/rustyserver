use std::collections::HashMap;
use std::io::{ErrorKind};
use std::ops::Add;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct ConfigurationForm {
    address: Option<String>,
    port: Option<u16>,
    pages: Option<HashMap<String, String>>
}

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub pages: HashMap<String, String>
}

pub fn load() -> Result<Config, ErrorKind> {

    let read_config: ConfigurationForm = read_file("config.toml")?;

    let ip_address: String = match read_config.address.clone() {
        None => panic!("No ip-address was provided in the config file!"),
        Some(ip_address) => ip_address
    };

    let port: String = match read_config.port.clone() {
        None => panic!("No port was provided in the config file!"),
        Some(port) => port.to_string()
    };

    let config: Config = Config {
        address: ip_address.add(":").add(&port),
        pages: match read_config.pages {
            None => HashMap::new(),
            Some(pages) => pages
        }
    };

    Ok(config)
}

fn read_file(filepath: &str) -> Result<ConfigurationForm, ErrorKind> {

    let data_string = match std::fs::read_to_string(filepath) {
        Ok(file) => file,
        Err(error) => return Err(error.kind())
    };

    let config: ConfigurationForm = match toml::from_str(&data_string) {
        Ok(config) => config,
        Err(_) => return Err(ErrorKind::Other)
    };

    Ok(config)
}