use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use super::discovery::DeviceInfo;

/// デバイスに接続してTcpStreamを返す
pub async fn connect_to_device(device_info: &DeviceInfo) -> Result<TcpStream, String> {
    let DeviceInfo { device_name, ip_address, port } = device_info;
    
    let ip: IpAddr = ip_address.parse().map_err(|e: std::net::AddrParseError| e.to_string())?;
    let socket_addr = SocketAddr::new(ip, *port);
    
    println!("Connecting to device: {} at {}:{}", device_name, ip_address, port);
    
    let stream = timeout(
        Duration::from_secs(5),
        TcpStream::connect(socket_addr)
    ).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
    
    println!("Connected to device: {}", device_name);
    
    Ok(stream)
}
