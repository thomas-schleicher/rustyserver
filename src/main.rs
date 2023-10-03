
mod config;

use::std::net::{TcpListener, TcpStream};
use std::fs;
use std::io::{BufRead, BufReader, Write};


fn main() {


    let config = config::load("config.toml").unwrap();
    println!("{:?}", config);

    let address = "127.0.0.1:8888";

    let tcp_listener: TcpListener;
    match TcpListener::bind(address) {
        Ok(listener) => tcp_listener = listener,
        Err(_error) => panic!("Error: Cannot bind listener to {}!", address)
    }

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(_error) => panic!("Error: Connection failed!")
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    match stream.write_all(response.as_bytes()) {
        Err(_error) => panic!("Error: Response failed!"),
        Ok(()) => println!("Response sent!")
    }
}
