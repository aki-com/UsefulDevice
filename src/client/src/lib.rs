use discover_connect::discover_server;
use std::collections::HashMap;
use std::net::IpAddr;
use tokio::net::TcpStream;
use std::sync::Arc;
mod discover_connect;
mod send_cmdID;
use crate::discover_connect::ServerMap;
use tokio::sync::RwLock;

pub fn client_test(){
    println!("client_test");
}

pub async fn get_server() -> ServerMap {
    let servers: ServerMap = Arc::new(RwLock::new(HashMap::new()));
    discover_server(servers.clone()).await; // 非同期呼び出し
    servers
}

pub async fn change_server(server_map: (String, IpAddr, u16)) {
    let (_name, ip, port) = server_map.clone();
    
    // tokio を使って非同期接続
    if let Some(stream) = discover_connect::connect_to_server(ip, port).await {
        // 非同期処理なので、await を使う
        send_cmdID::communication_loop(stream).await;
    } else {
        println!("サーバーへの接続に失敗しました。");
    }
}