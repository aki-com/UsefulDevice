

// Link library - Network utilities for device communication

mod tcp;
mod mdns;


pub use tcp::{TcpConnection, TcpServer, TcpError};
use mdns::{Mdns, Device};


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
    let manager = Mdns::new()?;
    manager.discover(timeout_secs).await
}

/// mDNSサービス登録
pub async fn register_mdns_service(port: u16) -> Result<(), String> {
    let manager = Mdns::new()?;
    let _event_loop = manager.register(port).await
        .map_err(|e| e.to_string())?;
    Ok(())
}
