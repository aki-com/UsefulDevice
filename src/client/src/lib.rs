use discover_connect::discover_server;
use std::collections::HashMap;
use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::sync::Mutex; 
mod discover_connect;
mod send_cmd;

use std::sync::Arc;
use once_cell::sync::Lazy;

static STREAM: Lazy<Arc<Mutex<Option<TcpStream>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));


pub fn client_test(){
    println!("client_test");
}

pub async fn get_server() -> HashMap<usize, (String, IpAddr, u16)> {
    discover_server().await

}
pub async fn change_server(server_map: (String, IpAddr, u16)) {
    
    let (_name, ip, port) = server_map.clone();

        // tokio を使って非同期接続
        if let Some(new_stream) = discover_connect::connect_to_server(ip, port).await {
            let mut stream_guard = STREAM.lock().await;
            *stream_guard = Some(new_stream);
        } else {
            println!("サーバーへの接続に失敗しました。");
        }
}

pub async fn send_command(input: String) {
    let mut stream_guard = STREAM.lock().await;

    if let Some(ref mut stream) = *stream_guard {
            send_cmd::process_input(stream, &input).await;
    } else {
        println!("接続されていません。");
    }
}
