use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, interval};

/// 指定されたストリームにメッセージを送信
pub async fn send_to_client(stream: &Arc<Mutex<TcpStream>>, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut locked_stream = stream.lock().await;
    locked_stream.write_all(format!("{}\n", message).as_bytes()).await?;
    locked_stream.flush().await?;
    Ok(())
}

/// 定期的にステータスを送信するタスク
pub async fn periodic_status_sender(stream: Arc<Mutex<TcpStream>>) {
    let mut interval = interval(Duration::from_secs(10));
    let mut counter = 0;
    
    loop {
        interval.tick().await;
        counter += 1;
        
        let status_message = format!("Status update #{}: Server is running", counter);
        
        if let Err(e) = send_to_client(&stream, &status_message).await {
            eprintln!("Failed to send status: {}", e);
            break;
        }
        
        println!("Sent status update to client");
    }
}

/// キー操作結果を通知
pub async fn notify_key_result(stream: &Arc<Mutex<TcpStream>>, key_combination: &str, success: bool) {
    let message = if success {
        format!("Key combination '{}' executed successfully", key_combination)
    } else {
        format!("Failed to execute key combination '{}'", key_combination)
    };
    
    if let Err(e) = send_to_client(stream, &message).await {
        eprintln!("Failed to send key result notification: {}", e);
    }
}
