use crate::tcpclient::tcpclient_impl::TcpClient;
use crate::tcpserver::tcpserver_impl::TcpServer;
use std::thread::{self, sleep};
use std::time::Duration;
use std::sync::Arc;

pub struct TcpInterface {
    listening_port: String,
    tcp_server: Arc<TcpServer>,
    tcp_client: TcpClient,
}

impl TcpInterface {
    pub fn new(listening_port: String) -> TcpInterface {
        let listening_port_var = listening_port.clone();
        let tcp_server = Arc::new(TcpServer::new(listening_port));
        let tcp_client = TcpClient::new();
        TcpInterface {
            listening_port: listening_port_var,
            tcp_client: tcp_client,
            tcp_server: tcp_server
        }
    }

    /// Creates a listening thread and sends a simple message
    pub fn run_ex(&self) {
        let server = self.tcp_server.clone();
        thread::spawn(move|| {
            server.main_loop();
        });
        sleep(Duration::from_secs(5));
        match self.tcp_client.send_message(self.listening_port.clone()) {
            Ok(_) => {println!("Sent message to self!")},
            Err(e) => {println!("Got error! {e:#?}")}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e() {
        let interface = TcpInterface::new("127.0.0.1:7979".to_string());
        // test that a simple message sent from the TcpInterface's client is received by the
        // server.
        let server = interface.tcp_server.clone();
        thread::spawn(move|| {
            server.main_loop_testing();
        });
        match interface.tcp_client.send_message(interface.listening_port.clone()) {
            Ok(_) => {},
            Err(e) => {assert!(false, "Unexpected error: {e:?}")}
        };
    }
}