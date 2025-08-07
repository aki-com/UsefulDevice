

// Link library - Network utilities for device communication

mod tcp;
mod mdns;


pub use tcp::{TcpConnection, TcpServer, TcpError};
use mdns::{MdnsClient,Device};


/// TCPサーバーを起動して、指定されたポートでリッスン
pub async fn server_start(port: u16) -> Result<TcpServer, TcpError> {
    TcpServer::bind(port).await
}

/// TCPコネクションを受け入れ
pub async fn connection_accept(server: &TcpServer) -> Result<TcpConnection, TcpError> {
    server.accept().await
}

/// デバイス検索（mDNS）
pub async fn discover_devices(timeout_secs: f32) -> Result<Vec<Device>, String> {
    let manager = MdnsClient::new();
    manager.discover(timeout_secs).await
}



#[cfg(not(any(target_os = "android", target_os = "ios")))]
use mdns::MdnsServer;
/// mDNSサービス登録
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub async fn register_mdns_service(port: u16) -> Result<(), String> {
    let manager = MdnsServer::new()
        .map_err(|e| e.to_string())?;
    let _event_loop = manager.register(port).await
        .map_err(|e| e.to_string())?;
    Ok(())
}
