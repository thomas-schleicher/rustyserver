
mod config;

use::std::net::{TcpListener, TcpStream};
use std::{fs, thread};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use crate::config::Config;


fn main() {

    let config: Config = match config::load() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error.to_string())
    };

    let tcp_listener: TcpListener = match TcpListener::bind(&config.address) {
        Ok(listener) => listener,
        Err(_error) => panic!("Error: Cannot bind listener!")
    };

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::scope(|_| handle_connection(stream, &config.pages))
            }
            Err(_error) => panic!("Error: Connection failed!")
        }
    }
}

fn handle_connection(mut stream: TcpStream, pages: &HashMap<String, String>) {

    let buf_reader = BufReader::new(&mut stream);

    let request: String = buf_reader.lines().next().unwrap().unwrap(); // Still needs error handling
    let request_path = request.split(" ").nth(1).unwrap(); // also needs error handling

    let (status_line, file_path) = if !pages.contains_key(request_path) {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    } else {
        ("HTTP/1.1 200 OK", pages.get_key_value(request_path).unwrap().1.as_str())
    };

    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    match stream.write_all(response.as_bytes()) {
        Err(_error) => panic!("Error: Response failed!"),
        Ok(()) => println!("Response sent!")
    }
}
