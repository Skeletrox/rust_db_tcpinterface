// See how this is imported in mod.rs and main.rs
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::str::FromStr;
use protobuf::Message;

use rust_db_proto::proto::messages::Request;

pub struct TcpServer {
    listening_port: String
}

// The lack of pub means this function is internal only
fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut dest: Vec<u8> = Vec::new();
    let num_bytes = buf_reader.read_to_end(&mut dest).unwrap();
    
    println!("Num bytes: {num_bytes:#?}");
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();

    let msg = Request::parse_from_bytes(&dest);
    println!("Msg: {msg:#?}");
}

#[cfg(test)]
// This is a validation for testing purposes
fn validate_parsed_value(mut stream: TcpStream) {
    use rust_db_proto::proto::messages::RequestType;
    let mut buf_reader = BufReader::new(&mut stream);
    let mut dest: Vec<u8> = Vec::new();
    Read::read_to_end(&mut buf_reader, &mut dest).unwrap();
    let msg = Request::parse_from_bytes(&dest).unwrap();
    assert_eq!(msg.req_type.unwrap(), RequestType::PING);
    assert_eq!(msg.payload.unwrap(), "Hello");
}

impl TcpServer {

    pub fn new_generic() -> TcpServer {
        return TcpServer {
            listening_port: String::from_str("127.0.0.1:7878").unwrap()
         };
    }

    pub fn new(listening_port: String) -> TcpServer {
        return TcpServer {
            listening_port: listening_port
        }
    }

    pub fn main_loop(&self) {
        let listener = TcpListener::bind(self.listening_port.as_str()).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    }

    #[cfg(test)]
    pub fn main_loop_testing(&self) {
        let listener = TcpListener::bind(self.listening_port.as_str()).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            validate_parsed_value(stream);
        }
    }
}