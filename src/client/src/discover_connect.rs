use tokio::net::TcpStream;
use std::collections::HashMap;
use std::net::IpAddr;
use zeroconf_tokio::{MdnsBrowser, ServiceType};
use zeroconf_tokio::MdnsBrowserAsync;
use zeroconf_tokio::prelude::*;
use tokio::sync::oneshot;
use tokio::time::{timeout, Duration};
use std::thread;


fn blocking_discover_server() -> HashMap<usize, (String, IpAddr, u16)> {
    let service_type = ServiceType::new("useful_devices", "udp").unwrap();
    let mut browser = MdnsBrowserAsync::new(MdnsBrowser::new(service_type)).unwrap();
    let mut servers = HashMap::new();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        browser.start().await.unwrap();
        let mut index = 1;
        println!("Searching for servers...");

        // タイムアウトを設定して探索を2秒で終了
        let result = timeout(Duration::from_secs(1), async {
            while let Some(Ok(service)) = browser.next().await {
                let ip_str = service.address();
                if let Ok(ip) = ip_str.parse::<IpAddr>() {
                    if ip.is_ipv4() {
                        let name = service.name().to_string();
                        if !servers.values().any(|(n, existing_ip, _)| existing_ip == &ip && n == &name) {
                            servers.insert(index, (name.clone(), ip, 5000));
                            println!("{}: {} {}", index, name, ip);
                            index += 1;
                        }
                    }
                }
            }
        }).await;

        if result.is_err() {
            println!("Discovery timed out after 2 seconds.");
        }
    });

    println!("Discovered {} servers.", servers.len());
    servers
}

pub async fn discover_server() -> HashMap<usize, (String, IpAddr, u16)> {
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        let result = blocking_discover_server();
        let _ = tx.send(result);
    });

    rx.await.unwrap_or_default()
}

/*pub fn select_server(servers: &HashMap<usize, (String, IpAddr, u16)>) -> Option<(IpAddr, u16)> {
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
}*/

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