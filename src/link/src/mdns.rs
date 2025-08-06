// mDNSサービス管理（最適化版）
use zeroconf_tokio::{MdnsService, ServiceType};
use zeroconf_tokio::prelude::*;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::timeout;

pub type Result<T> = std::result::Result<T, String>;

/// mDNSサービス管理（最適化版）
pub struct Mdns {
    service_type: ServiceType,
}

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

impl Mdns {
    pub fn new() -> Result<Self> {
        let service_type = ServiceType::new("useful_devices", "tcp")
            .map_err(|e| e.to_string())?;
        Ok(Self { service_type })
    }

    /// デバイス情報を作成
    pub fn create_device(&self, name: String, ip: IpAddr, port: u16) -> Device {
        Device::new(name, ip, port)
    }

    /// サービスを登録
    pub async fn register(&self, port: u16) -> Result<()> {
        let mut service = MdnsService::new(self.service_type.clone(), port);
        service.register()
            .map_err(|e| e.to_string())?;
        
        println!("mDNS service registered on port {}", port);
        Ok(())
    }

    /// デバイスを発見（mdns-sdベース）
    pub async fn discover(&self, timeout_secs: f32) -> Result<Vec<Device>> {
        let mdns = ServiceDaemon::new()
            .map_err(|e| format!("Failed to create mdns daemon: {}", e))?;
        
        let receiver = mdns
            .browse("_useful_devices._tcp.local.")
            .map_err(|e| format!("Failed to browse for mDNS services: {}", e))?;

        let mut devices = Vec::new();
        
        println!("Searching for devices...");

        let timeout_duration = Duration::from_secs_f32(timeout_secs);
        let start_time = Instant::now();

        while start_time.elapsed() < timeout_duration {
            // Wait up to 500ms for the next event
            match timeout(Duration::from_millis(500), receiver.recv_async()).await {
                Ok(Ok(ServiceEvent::ServiceResolved(info))) => {
                    // IPv4アドレスのみを取得
                    if let Some(ip) = info.get_addresses().iter().find(|addr| addr.is_ipv4()) {
                        let mut name = info.get_hostname().to_string();
                        if name.ends_with(".local.") {
                            name = name.trim_end_matches(".local.").to_string();
                        }
                        
                        // 重複チェック
                        if !devices.iter().any(|d: &Device| d.addr.ip() == *ip && d.name == name) {
                            let port = info.get_port();
                            let device = self.create_device(name.clone(), *ip, port);
                            devices.push(device);
                            println!("Found device: {} at {}:{}", name, ip, port);
                        }
                    }
                }
                Ok(_) => {}
                Err(_) => break, // Timeout on receiving
            }
        }

        println!("Discovery completed. Found {} devices", devices.len());
        Ok(devices)
    }
}

impl Default for Mdns {
    fn default() -> Self {
        Self::new().expect("Failed to create Mdns")
    }
}
