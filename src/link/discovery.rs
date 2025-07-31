use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// デバイス情報
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub device_name: String,
    pub ip_address: String,
    pub port: u16,
}

pub async fn get_devices() -> Vec<DeviceInfo> {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");
    let receiver = mdns
        .browse("_useful_devices._udp.local.")
        .expect("Failed to browse for mDNS services");

    let mut devices = Vec::new();

    println!("Searching for devices...");

    let timeout_duration = Duration::from_secs_f32(1.5);
    let start_time = Instant::now();

    while start_time.elapsed() < timeout_duration {
        match timeout(Duration::from_millis(500), receiver.recv_async()).await {
            Ok(Ok(ServiceEvent::ServiceResolved(info))) => {
                if let Some(ip) = info.get_addresses().iter().find(|addr| addr.is_ipv4()) {
                    let mut name = info.get_hostname().to_string();
                    if name.ends_with(".local.") {
                        name.truncate(name.len() - 7);
                    }
                    
                    devices.push(DeviceInfo {
                        device_name: name,
                        ip_address: ip.to_string(),
                        port: 5000, // デフォルトポート
                    });
                }
            }
            _ => {}
        }
    }

    devices
}
