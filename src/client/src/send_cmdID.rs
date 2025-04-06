use tokio::net::TcpStream;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}; // AsyncReadExt と AsyncWriteExt をインポート
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn communication_loop(stream: TcpStream) {
    let stream = Arc::new(Mutex::new(stream)); // スレッド間で共有
    let mut reader = io::BufReader::new(io::stdin()); // 非同期で入力を読む
    let mut input = String::new();
    println!("コマンド（数字）を入力するか、音量を変更するには 'volume <値>' と入力してください。または 'exit' で終了します:");

    loop {
        
        input.clear();
        if reader.read_line(&mut input).await.is_err() {
            eprintln!("入力の読み込みに失敗しました");
            continue;
        }

        let input = input.trim().to_string();
        if input == "exit" {
            println!("終了します...");
            break;
        }

        let stream_clone = Arc::clone(&stream);
        tokio::spawn(async move {
            let mut stream = stream_clone.lock().await; // ミューテックスをロックして stream を取り出し

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

            // サーバーからの応答を非同期で待機
            let mut buffer = vec![0; 512];
            match stream.read(&mut buffer).await {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        println!("サーバーの応答: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    } else {
                        println!("サーバーが接続を閉じました。");
                    }
                }
                Err(e) => eprintln!("サーバーからの読み込みに失敗しました: {}", e),
            }
        });
    }
}
