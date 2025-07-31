use std::net::IpAddr;
use tokio::net::TcpListener;
use super::{register_mdns_service, get_local_ip};

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
}

/// TCP リスナーを作成
pub async fn create_tcp_listener(ip: IpAddr, port: u16) -> Result<TcpListener, String> {
    let addr = std::net::SocketAddr::new(ip, port);
    TcpListener::bind(addr).await.map_err(|e| e.to_string())
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
    })
}
