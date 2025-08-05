

// Link library - Network utilities for device communication

mod tcp;
mod mdns;

pub use tcp::{TcpConnection, TcpServer, TcpError};
pub use mdns::{MdnsManager, Device, MdnsError};

// 後方互換性のため
pub use tcp::TcpConnection as TcpConnectionClass;

/// TCPサーバーを起動して、指定されたポートでリッスン
pub async fn server_start(port: u16) -> Result<TcpServer, TcpError> {
    TcpServer::bind(port).await
}

/// TCPコネクションを受け入れ
pub async fn connection_accept(server: &TcpServer) -> Result<TcpConnection, TcpError> {
    server.accept().await
}

/// デバイス検索（mDNS）
pub async fn discover_devices(timeout_secs: f32) -> Result<Vec<Device>, MdnsError> {
    let manager = MdnsManager::new()?;
    manager.discover_devices(timeout_secs).await
}

/// mDNSサービス登録
pub async fn register_mdns_service(port: u16) -> Result<(), MdnsError> {
    let manager = MdnsManager::new()?;
    manager.register_service(port).await
}
