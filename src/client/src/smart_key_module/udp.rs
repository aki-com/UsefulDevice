use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::net::{IpAddr};
use std::{thread, time, collections::HashMap};
use std::io;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;

pub fn discover_server() -> HashMap<usize, (String, IpAddr, u16)> {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");
    let receiver = mdns.browse("_smart_key._udp.local.").expect("Failed to browse for mDNS services");

    let mut servers = HashMap::new();
    let mut index = 1;

    println!("Searching for servers...");

    let timeout = Duration::from_secs_f32(1.5); // タイムアウトを1.5秒に設定
    let start_time = Instant::now();

    while start_time.elapsed() < timeout {
        if let Ok(event) = receiver.recv_timeout(Duration::from_secs_f32(0.5)) {
            if let ServiceEvent::ServiceResolved(info) = event {
                if let Some(ip) = info.get_addresses().iter().next() {
                    let mut name = info.get_hostname().to_string();
                    if name.ends_with(".local.") {
                        name = name.trim_end_matches(".local.").to_string();
                    }
                    if !servers.values().any(|(n, existing_ip, _)| existing_ip == ip && n == &name) {
                        servers.insert(index, (name.clone(), *ip, 5000)); // サーバー情報を格納
                        println!("{}: {} {}", index, name, ip);
                        index += 1;
                    }
                }
            }
        }
    }

    servers
}

// UDP接続のみ行う
pub async fn connect_to_server(ip: IpAddr, port: u16) -> Option<UdpSocket> {
    println!("Trying to connect to server at {}:{}", ip, port);

    // UdpSocketでサーバーに接続
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap(); // 任意のローカルポートにバインド
    let addr = (ip, port);

    // UDPでデータを送信して、接続ができるかチェック（自動的に接続完了）
    let msg = b"Hello, Server!";
    match socket.send_to(msg, addr).await {
        Ok(_) => {
            println!("Message sent to server at {}:{}", ip, port);
            Some(socket) // 接続したソケットを返す
        }
        Err(e) => {
            eprintln!("Failed to send message to server: {}", e);
            None
        }
    }
}

// 自動でサーバーに接続する関数
pub async fn auto_connect(servers: &HashMap<usize, (String, IpAddr, u16)>) -> Option<UdpSocket> {
    if let Some((ip, port)) = servers.get(&1).map(|(_, ip, port)| (*ip, *port)) { // 自動で1番目のサーバーに接続する例
        println!("Automatically connecting to server at {}:{}", ip, port);
        connect_to_server(ip, port).await
    } else {
        println!("No available server to connect to.");
        None
    }
}
