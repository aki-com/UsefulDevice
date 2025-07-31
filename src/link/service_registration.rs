use std::net::IpAddr;
use local_ip_address::list_afinet_netifas;
use zeroconf_tokio::MdnsServiceAsync;
use zeroconf_tokio::{MdnsService, ServiceType};
use zeroconf_tokio::prelude::*;
use zeroconf_tokio::bonjour::event_loop::BonjourEventLoop;

/// mDNSサービスを登録してイベントループを返す
pub async fn register_mdns_service() -> Result<BonjourEventLoop, Box<dyn std::error::Error + Send + Sync>> {
    let service_type = ServiceType::new("useful_devices", "udp")?;
    let mut service = MdnsService::new(service_type, 8080);
    let event_loop = service.register()?;
    let mut service = MdnsServiceAsync::new(service)?;
    let result = service.start().await?;
    println!("Service registration started: {:?}", result);
    Ok(event_loop)
}

/// 適切なローカルIPv4アドレスを取得
pub fn get_local_ip() -> Result<IpAddr, Box<dyn std::error::Error + Send + Sync>> {
    let ip: Option<IpAddr> = list_afinet_netifas()?
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
                    IpAddr::V4(ipv4) => Some(IpAddr::V4(ipv4)),
                    _ => None,
                })
        });

    ip.ok_or_else(|| "No suitable IPv4 address found".into())
}
