use discover_connect::discover_server;
use std::collections::HashMap;
use std::net::IpAddr;
use tokio::net::TcpStream;

mod discover_connect;
mod send_cmdID;

pub fn client_test(){
    println!("client_test");
}

pub fn get_server() -> HashMap<usize, (String, IpAddr, u16)> {
    discover_server()

}

pub async fn change_server(server_map: (String, IpAddr, u16)) {
    
    let (_name, ip, port) = server_map.clone();
    
    
    // tokio を使って非同期接続
    if let Some(stream) = discover_connect::connect_to_server(ip, port).await {
        // 非同期処理なので、await を使う
        // ここで stream を使って通信を行う
        send_cmdID::communication_loop(stream).await;
        
    } else {
        println!("サーバーへの接続に失敗しました。");
    }
}