use mdns_sd::{ServiceDaemon, ServiceInfo};
use hostname;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use local_ip_address::list_afinet_netifas;
use std::net::TcpListener;
use std::io::Write;

pub fn start_server() {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");

    // デバイスの名前を取得
    let device_name = hostname::get()
        .expect("Failed to get hostname")
        .to_string_lossy()
        .into_owned();
    
    println!("Device Name: {}", device_name); 

    // ローカルIPv4アドレスを取得
    let ip: Option<IpAddr> = list_afinet_netifas()
        .expect("Failed to get local interfaces")
        .into_iter()
        .find_map(|(_, ip)| match ip {
            IpAddr::V4(ipv4) => Some(IpAddr::V4(ipv4)), // IPv4のみ取得
            _ => None,
        });

    let ip = ip.expect("No IPv4 address found");

    println!("Local IP: {}", ip);

    // mDNSサービスを登録
    let service_info = ServiceInfo::new(
        "_useful_devices._udp.local.", // サービス名
        &device_name,                  // インスタンス名
        &format!("{}.local.", device_name), // ホスト名 (FQDN)
        ip,                             // IPアドレス
        5000,                           // ポート番号
        HashMap::new(),                  // TXTレコード（今回は空）
    )
    .expect("Failed to create mDNS service info");

    let _service_handle = mdns.register(service_info).expect("Failed to register mDNS service");

    println!("mDNS service registered: {} on {}", device_name, ip);

    // サーバーのポート5000でリッスン開始
    match TcpListener::bind((ip, 5000)) {
        Ok(listener) => {
            println!("Server is listening on {}:5000", ip);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        println!("Connection established with: {}", stream.peer_addr().unwrap());
                        let _ = stream.write_all(b"Hello from server!");  // write_allを使用
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to bind to port 5000: {}", e);
        }
    }

    // サービスを維持
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}