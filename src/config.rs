
use std::io::{Error};
use std::net::{IpAddr, Ipv4Addr};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Configuration {
    address: IpAddr,
    port: u16,
    paths: Option<String>
}



pub fn load(filepath: std::path) -> Result<Configuration, Error> {




    let c: Configuration = Configuration {
        address: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
        port: 0,
        paths: None,
    };

    Ok(c)
}

pub fn create() {

}