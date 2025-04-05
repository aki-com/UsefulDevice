use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::net::{IpAddr};
use std::{thread, time, collections::HashMap};
use std::io;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::task;
use tokio::time::sleep;
use std::sync::Arc;


pub type ServerMap = Arc<RwLock<HashMap<usize, (String, IpAddr, u16)>>>;

pub async fn discover_server(servers: ServerMap) {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");
    let receiver = mdns
        .browse("_useful_devices._udp.local.")
        .expect("Failed to browse for mDNS services");

    let mut index = 1;

    loop {
        // 0.5秒待機（recv_timeout相当）
        match task::spawn_blocking({
            let receiver = receiver.clone();
            move || receiver.recv_timeout(Duration::from_secs_f32(0.5))
        })
        .await
        {
            Ok(Ok(ServiceEvent::ServiceResolved(info))) => {
                if let Some(ip) = info.get_addresses().iter().next() {
                    let mut name = info.get_hostname().to_string();
                    if name.ends_with(".local.") {
                        name = name.trim_end_matches(".local.").to_string();
                    }

                    let mut servers_lock = servers.write().await;
                    let exists = servers_lock.values().any(|(n, existing_ip, _)| {
                        existing_ip == ip && n == &name
                    });

                    if !exists {
                        servers_lock.insert(index, (name.clone(), *ip, 5000));
                        println!("{}: {} {}", index, name, ip);
                        index += 1;
                    }
                }
            }
            _ => {} // タイムアウトまたはエラーは無視
        }

        // ループ間隔（あまり詰めすぎない）
        sleep(Duration::from_millis(100)).await;
    }
}

pub fn select_server(servers: &HashMap<usize, (String, IpAddr, u16)>) -> Option<(IpAddr, u16)> {
    if servers.is_empty() {
        return None;
    }

    loop {
        println!("Enter the number of the server you want to connect to:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if let Ok(choice) = input.trim().parse::<usize>() {
            if let Some((_name, ip, port)) = servers.get(&choice) {
                return Some((*ip, *port));
            }
        }
        println!("Invalid selection. Try again.");
    }
}

// 接続だけを行う
pub async fn connect_to_server(ip: IpAddr, port: u16) -> Option<TcpStream> {
    println!("Trying to connect to server at {}:{}", ip, port);
    match tokio::net::TcpStream::connect((ip, port)).await {
        Ok(stream) => {
            println!("Connected to server at {}:{}", ip, port);
            Some(stream)
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            None
        }
    }
}