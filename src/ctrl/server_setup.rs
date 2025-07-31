use std::net::IpAddr;
use ud_link::{register_mdns_service, get_local_ip};
use crate::connection_manager::{ConnectionManager, create_tcp_listener};
use tokio::net::TcpListener;

/// サーバー設定
pub struct ServerConfig {
    pub port: u16,
    pub welcome_message: String,
    pub reject_message: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 5000,
            welcome_message: "Hello from server!".to_string(),
            reject_message: "Server busy".to_string(),
        }
    }
}

/// サーバー初期化結果
pub struct ServerSetup {
    pub ip: IpAddr,
    pub listener: TcpListener,
    pub connection_manager: ConnectionManager,
}

/// サーバーを初期化（mDNS + IP取得 + TCPリスナー）
pub async fn initialize_server(config: ServerConfig) -> Result<ServerSetup, String> {
    // mDNSサービスを登録
    let _event_loop = register_mdns_service().await
        .map_err(|e| format!("Failed to start mDNS service: {}", e))?;

    // ローカルIPを取得
    let ip = get_local_ip()
        .map_err(|e| format!("Failed to get local IP: {}", e))?;

    println!("Selected IP: {}", ip);

    // TCPリスナーを作成
    let listener = create_tcp_listener(ip, config.port).await?;
    println!("Server is listening on {}:{}", ip, config.port);

    Ok(ServerSetup {
        ip,
        listener,
        connection_manager: ConnectionManager::new(),
    })
}
