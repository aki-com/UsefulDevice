// TCP接続とサーバー機能（最適化版）
use std::net::{IpAddr, SocketAddr};
use local_ip_address::list_afinet_netifas;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum TcpError {
    #[error("Connection failed: {0}")]
    Connection(#[from] std::io::Error),
    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] std::net::AddrParseError),
    #[error("Timeout: {0}")]
    Timeout(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
}

pub type Result<T> = std::result::Result<T, TcpError>;

/// TCP接続管理（最適化版）
pub struct TcpConnection {
    stream: TcpStream,
    peer_addr: SocketAddr,
}

impl TcpConnection {
    pub fn new(stream: TcpStream, peer_addr: SocketAddr) -> Self {
        Self { stream, peer_addr }
    }

    /// 接続を作成
    pub async fn connect(addr: SocketAddr) -> Result<Self> {
        let stream = tokio::time::timeout(
            Duration::from_secs(5),
            TcpStream::connect(addr)
        ).await
            .map_err(|_| TcpError::Timeout("Connection timeout".to_string()))??;
        
        let peer_addr = stream.peer_addr()?;
        Ok(Self::new(stream, peer_addr))
    }

    /// 行単位でデータを送信
    pub async fn send_line(&mut self, data: &str) -> Result<()> {
        self.stream.write_all(data.as_bytes()).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await?;
        Ok(())
    }

    /// 行単位でデータを受信
    pub async fn receive_line(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        let mut byte = [0u8; 1];
        
        loop {
            match self.stream.read_exact(&mut byte).await {
                Ok(_) => {
                    if byte[0] == b'\n' {
                        break;
                    }
                    if byte[0] != b'\r' {
                        buf.push(byte[0]);
                    }
                }
                Err(e) => return Err(TcpError::Connection(e)),
            }
        }
        
        String::from_utf8(buf)
            .map_err(|e| TcpError::Protocol(format!("Invalid UTF-8: {}", e)))
    }

    /// バイナリデータを送信
    pub async fn send_bytes(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data).await?;
        self.stream.flush().await?;
        Ok(())
    }

    /// バイナリデータを受信
    pub async fn receive_bytes(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let bytes_read = self.stream.read(buffer).await?;
        Ok(bytes_read)
    }

    /// ピアアドレスを取得
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }
}

/// TCPサーバー（最適化版）
pub struct TcpServer {
    listener: TcpListener,
    local_addr: SocketAddr,
}

impl TcpServer {
    /// サーバーを作成
    pub async fn bind(port: u16) -> Result<Self> {
        let ip = get_local_ip()?;
        let addr = SocketAddr::new(ip, port);
        let listener = TcpListener::bind(addr).await?;
        let local_addr = listener.local_addr()?;
        
        println!("TCP Server bound to {}", local_addr);
        Ok(Self { listener, local_addr })
    }

    /// 接続を受け入れ
    pub async fn accept(&self) -> Result<TcpConnection> {
        let (stream, peer_addr) = self.listener.accept().await?;
        println!("Accepted connection from {}", peer_addr);
        Ok(TcpConnection::new(stream, peer_addr))
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}

/// ローカルIPアドレスを取得（192.168.x.x を優先）
pub fn get_local_ip() -> Result<IpAddr> {
    let interfaces = list_afinet_netifas()
        .map_err(|e| TcpError::Protocol(format!("Failed to get interfaces: {}", e)))?;

    // 192.168.x.x を優先
    for (_, ip) in &interfaces {
        if let IpAddr::V4(ipv4) = ip {
            if ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168 {
                return Ok(IpAddr::V4(*ipv4));
            }
        }
    }

    // 他のプライベートアドレス
    for (_, ip) in &interfaces {
        if let IpAddr::V4(ipv4) = ip {
            let octets = ipv4.octets();
            // 10.x.x.x または 172.16-31.x.x
            if octets[0] == 10 || (octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31) {
                return Ok(IpAddr::V4(*ipv4));
            }
        }
    }

    // ローカルアドレス以外
    for (_, ip) in &interfaces {
        if !ip.is_loopback() {
            return Ok(*ip);
        }
    }

    Err(TcpError::Protocol("No suitable network interface found".to_string()))
}
