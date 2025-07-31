use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::MutexGuard;
use ud_ctrl::send_key_combination;

/// クライアントからの接続を処理
pub async fn handle_client(mut stream: MutexGuard<'_, TcpStream>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                let parts: Vec<&str> = received.split('+').collect();
                println!("command: {:?}", parts);
                
                match send_key_combination(&parts) {
                    Ok(_) => {
                        println!("Key combination executed successfully");
                    }
                    Err(e) => {
                        eprintln!("Failed to execute key combination: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}
