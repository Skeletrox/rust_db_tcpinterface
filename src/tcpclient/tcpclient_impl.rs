use std::io::Bytes;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use prost::Message;
use rust_db_proto::proto::{Request, RequestType};


pub struct TcpClient {

}

impl TcpClient {
    pub fn new() -> TcpClient {
        TcpClient {}
    }

    pub async fn send_message(&self, addr: String) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(addr).await?;
        let mut request = Request::default();
        request.set_req_type(RequestType::Ping);
        request.payload = Some("Hello".to_string());
        println!("Sending: {request:#?}");
        let mut bytes = vec![];
        request.encode_length_delimited(&mut bytes).unwrap();
        let n = request.encoded_len();
        println!("Bytes: {bytes:#?}, len: {n:#?}");
        stream.write_all(&bytes[..]).await?;
        Ok(())
    }

}