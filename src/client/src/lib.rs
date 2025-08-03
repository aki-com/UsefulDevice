use discover_connect::discover_server;
use std::collections::HashMap;
use std::net::IpAddr;
use tokio::net::TcpStream;

mod discover_connect;
mod send_cmd;

pub struct Client {
    stream: Option<TcpStream>,
}

impl Client {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub async fn get_servers() -> HashMap<usize, (String, IpAddr, u16)> {
        discover_server().await
    }

    pub async fn connect(server_info: (String, IpAddr, u16)) -> Result<Self, String> {
        let (_name, ip, port) = server_info;
        
        match discover_connect::connect_to_server(ip, port).await {
            Some(stream) => Ok(Self { stream: Some(stream) }),
            None => Err("サーバーへの接続に失敗しました".to_string())
        }
    }

    pub async fn send_command(&mut self, input: &str) -> Result<String, String> {
        match &mut self.stream {
            Some(stream) => {
                send_cmd::process_input(stream, input).await;
                Ok("送信完了".to_string())
            }
            None => Err("接続されていません".to_string())
        }
    }

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }
}

pub async fn get_server() -> HashMap<usize, (String, IpAddr, u16)> {
    Client::get_servers().await
}
