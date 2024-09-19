// See how this is imported in mod.rs and main.rs
use std::str::FromStr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(test)]
use rust_db_proto::proto::Request;

#[cfg(test)]
use prost::Message;

pub struct TcpServer {
    listening_port: String
}


#[cfg(test)]
// This is a validation for testing purposes
fn validate_parsed_value(msg: Request) -> bool {
    use rust_db_proto::proto::RequestType;
    return msg.req_type() == RequestType::Ping && msg.payload.unwrap() == "Hello";
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

    pub async fn main_loop(&self) -> Result<(), Box<dyn std::error::Error>>{
        let listener = TcpListener::bind(self.listening_port.as_str()).await?;

        println!("Server listening at: {}", self.listening_port.as_str());

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("Got connection from: {}", addr);
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];

                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // Connection closed
                    Ok(n) => {
                        println!("Received: {:?}", &buf[..n]);

                        // Echo this message back
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("Could not write back to socket: {:?}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error! {e:?}");
                    }
                }
            });
        }
    }

    #[cfg(test)]
    pub async fn main_loop_test(&self) -> Result<(), Box<dyn std::error::Error>>{

        let listener = TcpListener::bind(self.listening_port.as_str()).await?;
        println!("Server listening at: {}", self.listening_port.as_str());
        let (mut socket, addr) = listener.accept().await?;
        println!("Got connection from: {}", addr);
        let mut buf = vec![0; 1024];
        match socket.read(&mut buf).await {
            Ok(n) if n == 0 => Err("Connection closed!".into()), // Connection closed
            Ok(n) => {
                println!("Start: {:?}", &buf[..n]);
                let request = match Request::decode_length_delimited(&buf[..]) {
                    Ok(req) => req,
                    Err(e) => {
                        return Err(e.to_string().into());
                    }
                };
                if !validate_parsed_value(request) {
                    return Err("Validation failed".into());
                };
                println!("Done");
                return Ok(());
            },
            Err(e) => return Err(e.to_string().into())
        }
    }
}