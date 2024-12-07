use crate::tcpclient::tcpclient_impl::TcpClient;
use crate::tcpserver::tcpserver_impl::TcpServer;
use std::thread::sleep;
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
    pub async fn run_ex(&self) {
        let server = self.tcp_server.clone();
        tokio::spawn(async move {
            server.main_loop().await.unwrap();
        });
        sleep(Duration::from_secs(5));
        match self.tcp_client.send_message(self.listening_port.clone()).await {
            Ok(_) => {println!("Sent message to self!")},
            Err(e) => {println!("Got error! {e:#?}")}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[tokio::test]
    async fn test_e2e() {
        let addr_val = "127.0.0.1:7979".to_string();
        let server_address = addr_val.clone();
        let mutable_result = Arc::new(Mutex::new(true));
        let result_copy = mutable_result.clone();
        let value = tokio::spawn(async move {
            let server = TcpServer::new(server_address);
            // Any errors thrown should be propagated to the top
            let res = match server.main_loop_test().await {
                Ok(_) => {true},
                Err(e) => {
                    eprintln!("Main loop error: {e:?}");
                    false
                }
            };
            let mut t = result_copy.lock().unwrap();
            *t = res;
        });
        // wait for a bit before triggering a client
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let client = TcpClient::new();
        let _ = client.send_message(addr_val).await;
        while !value.is_finished() {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        let final_result = mutable_result.lock().unwrap();
        assert!(*final_result, "Check main loop for errors!");
    }
}