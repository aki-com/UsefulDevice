// mDNSサービス管理（最適化版）
use zeroconf_tokio::{MdnsService, ServiceType};
use zeroconf_tokio::prelude::*;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, thiserror::Error)]
pub enum MdnsError {
    #[error("Service registration failed: {0}")]
    Registration(String),
    #[error("Discovery failed: {0}")]
    Discovery(String),
    #[error("Service type creation failed: {0}")]
    ServiceType(String),
}

pub type Result<T> = std::result::Result<T, MdnsError>;

/// デバイス情報
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub name: String,
    pub addr: SocketAddr,
}

impl Device {
    pub fn new(name: String, ip: IpAddr, port: u16) -> Self {
        Self {
            name,
            addr: SocketAddr::new(ip, port),
        }
    }
}

/// mDNSサービス管理（最適化版）
pub struct MdnsManager {
    service_type: ServiceType,
}

impl MdnsManager {
    pub fn new() -> Result<Self> {
        let service_type = ServiceType::new("useful_devices", "tcp")
            .map_err(|e| MdnsError::ServiceType(e.to_string()))?;
        Ok(Self { service_type })
    }

    /// サービスを登録
    pub async fn register_service(&self, port: u16) -> Result<()> {
        let mut service = MdnsService::new(self.service_type.clone(), port);
        service.register()
            .map_err(|e| MdnsError::Registration(e.to_string()))?;
        
        println!("mDNS service registered on port {}", port);
        Ok(())
    }

    /// デバイスを発見（簡易版）
    pub async fn discover_devices(&self, _timeout_secs: f32) -> Result<Vec<Device>> {
        // 現在のzeroconf-tokioでは直接のブラウジングAPIが制限されているため、
        // 一旦空のリストを返す（将来的な拡張のため）
        println!("Device discovery requested (implementation pending)");
        Ok(Vec::new())
    }
}

impl Default for MdnsManager {
    fn default() -> Self {
        Self::new().expect("Failed to create MdnsManager")
    }
}
