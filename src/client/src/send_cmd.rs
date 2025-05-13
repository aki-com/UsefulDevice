use tokio::net::TcpStream;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;

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

pub async fn send_command(stream: std::sync::Arc<tokio::sync::Mutex<TcpStream>>, input: String) {
    let mut stream = stream.lock().await;
    process_input(&mut stream, &input).await;
}

pub async fn communication_loop(stream: TcpStream) {
    let stream = Arc::new(Mutex::new(stream));
    let mut reader = io::BufReader::new(io::stdin());
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

        // スレッドを生成せず、直接メインスレッドで処理する
        let mut stream_locked = stream.lock().await;
        process_input(&mut stream_locked, &input).await;
        // ここでロックが自動的に解放される
    }
}