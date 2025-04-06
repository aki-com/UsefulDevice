use mdns_sd::{ServiceDaemon, ServiceInfo};
use hostname;
use std::collections::HashMap;
use std::net::IpAddr;
use local_ip_address::list_afinet_netifas;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use crate::device_ctrl::handle_client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_server() {
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
        IpAddr::V4(ipv4) if ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168 => Some(IpAddr::V4(ipv4)),
        _ => None,
    })
    .or_else(|| {
        list_afinet_netifas()
            .expect("Failed to get local interfaces")
            .into_iter()
            .find_map(|(_, ip)| match ip {
                IpAddr::V4(ipv4) if ipv4.octets()[0] == 10 => Some(IpAddr::V4(ipv4)),
                _ => None,
            })
    })
    .or_else(|| {
        list_afinet_netifas()
            .expect("Failed to get local interfaces")
            .into_iter()
            .find_map(|(_, ip)| match ip {
                IpAddr::V4(ipv4) => Some(IpAddr::V4(ipv4)), // 何でもいいからIPv4を取得
                _ => None,
            })
    });

    let ip = ip.expect("No suitable IPv4 address found");
    println!("Selected IP: {}", ip);

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
    let listener = TcpListener::bind((ip, 5000)).await.unwrap();
    println!("Server is listening on {}:5000", ip);
    
    let current_connection: Arc<Mutex<Option<Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(None));

    while let Ok((stream, addr)) = listener.accept().await {
        let current_connection_clone = Arc::clone(&current_connection); // ここでクローンする
    
        let mut conn_guard = current_connection.lock().await;
    
        if conn_guard.is_some() {
            println!("Rejected connection from {}: already connected", addr);
            drop(conn_guard);
            let mut reject_stream = stream;
            let _ = reject_stream.write_all(b"Server busy\n").await;
            let _ = reject_stream.shutdown().await;
            continue;
        }
    
        println!("Connection established with: {}", addr);
    
        let stream: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(stream));
        *conn_guard = Some(Arc::clone(&stream)); // 接続状態を記録
        drop(conn_guard);
    
        {
            let mut locked_stream = stream.lock().await;
            let _ = locked_stream.write_all(b"Hello from server!").await;
        }
    
        let stream_clone = Arc::clone(&stream);
    
        tokio::spawn(async move {
            let _ = handle_client(stream_clone.lock().await).await;
    
            // クライアント切断時に current_connection をクリア
            let mut conn_guard = current_connection_clone.lock().await;
            *conn_guard = None;
            println!("Client disconnected, ready for new connection");
        });
    }

    // サービスを維持
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}