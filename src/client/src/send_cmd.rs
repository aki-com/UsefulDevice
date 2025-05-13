use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn process_input(stream: &mut TcpStream, input: &str) {
    // コマンド送信部分は変更なし
    if let Some(volume_command) = input.strip_prefix("volume ") {
        if let Ok(volume_change) = volume_command.trim().parse::<f32>() {
            if let Err(e) = stream.write_all(format!("volume {}\n", volume_change).as_bytes()).await {
                eprintln!("データの送信に失敗しました: {}", e);
                return;
            }
        } else {
            println!("有効な音量調整値を入力してください。");
            return;
        }
    } else if let Ok(num) = input.parse::<u32>() {
        if let Err(e) = stream.write_all(format!("{}\n", num).as_bytes()).await {
            eprintln!("データの送信に失敗しました: {}", e);
            return;
        }
    } else {
        println!("有効な数字、音量コマンド、または 'exit' で終了します。");
        return;
    }

    // 改善されたレスポンス読み取り部分
    let mut response = String::new();
    let mut buffer = [0u8; 1024];
    
    // タイムアウトを設定して読み取りを行う
    let timeout = tokio::time::timeout(
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
                        
                        // レスポンスの終わりを検出（例えば改行で終わるなど）
                        // サーバーの応答形式に合わせて調整が必要
                        if data.ends_with('\n') || response.len() > 4096 {
                            break;
                        }
                    },
                    Err(e) => {
                        eprintln!("サーバーからの読み込みに失敗しました: {}", e);
                        break;
                    }
                }
            }
        }
    ).await;

    match timeout {
        Ok(_) => {
            if !response.is_empty() {
                println!("サーバーの応答: {}", response.trim());
            }
        },
        Err(_) => println!("サーバーからの応答を待機中にタイムアウトしました。"),
    }
}

