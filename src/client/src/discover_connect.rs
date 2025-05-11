use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::net::{IpAddr};
use std::{thread, time, collections::HashMap};
use std::io;
use std::time::{Duration};
use tokio::net::TcpStream;
use tokio::time::{sleep, Instant};
use tokio::task;

pub async fn discover_server() -> HashMap<usize, (String, IpAddr, u16)> {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");
    let receiver = mdns.browse("_useful_devices._udp.local.").expect("Failed to browse for mDNS services");

    let mut servers = HashMap::new();
    let mut index = 1;

    println!("Searching for servers...");

    let timeout = Duration::from_secs_f32(1.5);
    let start_time = Instant::now();

    while Instant::now().duration_since(start_time) < timeout {
        let remaining = Duration::from_secs_f32(0.5);
        let recv_result = task::spawn_blocking({
            let receiver = receiver.clone();
            move || receiver.recv_timeout(remaining)
        }).await;

        if let Ok(Ok(ServiceEvent::ServiceResolved(info))) = recv_result {
            if let Some(ip) = info.get_addresses().iter().next() {
                let mut name = info.get_hostname().to_string();
                if name.ends_with(".local.") {
                    name = name.trim_end_matches(".local.").to_string();
                }
                if !servers.values().any(|(n, existing_ip, _)| existing_ip == ip && n == &name) {
                    servers.insert(index, (name.clone(), *ip, 5000));
                    println!("{}: {} {}", index, name, ip);
                    index += 1;
                }
            }
        }

        sleep(Duration::from_millis(50)).await;
    }

    servers
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