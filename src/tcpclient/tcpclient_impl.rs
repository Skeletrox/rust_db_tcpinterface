use std::io::prelude::*;
use std::net::TcpStream;
use protobuf::{EnumOrUnknown, Message};
use rust_db_proto::proto::messages::{RequestType, Request};


pub struct TcpClient {

}

impl TcpClient {
    pub fn new() -> TcpClient {
        TcpClient {}
    }

    pub fn send_message(&self, addr: String) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(addr).expect("Could not connect to host!");
        let mut request = Request::new();
        request.req_type = EnumOrUnknown::new(RequestType::PING);
        request.payload = Some("Hello".to_string());
        println!("Sending: {request:#?}");
        stream.write(&request.write_to_bytes().unwrap())?;
        Ok(())
    }
}