use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn send_command(stream: &mut TcpStream, input: &str) -> Result<String, String> {
    println!("Sending command: {}", input);
    
    stream.write_all(format!("{}\n", input.trim()).as_bytes()).await.map_err(|e| e.to_string())?;

    // レスポンス読み取り
    let mut response = String::new();
    let mut buffer = [0u8; 1024];
    
    let timeout_result = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        async {
            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => {
                        println!("サーバーが接続を閉じました。");
                        break;
                    },
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buffer[..n]);
                        response.push_str(&data);
                        
                        if data.ends_with('\n') || response.len() > 4096 {
                            break;
                        }
                    },
                    Err(e) => {
                        eprintln!("データの読み取りに失敗しました: {}", e);
                        return Err(e.to_string());
                    }
                }
            }
            Ok(())
        }
    ).await;

    match timeout_result {
        Ok(_) => {
            if !response.trim().is_empty() {
                println!("レスポンス: {}", response.trim());
                Ok(response.trim().to_string())
            } else {
                println!("レスポンスを受信しました。");
                Ok(String::new())
            }
        },
        Err(_) => {
            println!("レスポンスのタイムアウトが発生しました。");
            Err("Response timeout".to_string())
        }
    }
}
