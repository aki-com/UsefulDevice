use tokio::net::TcpStream;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn key_process_input(stream: &mut TcpStream, input: &str) {
    // 数字だけ送信
    if let Ok(num) = input.trim().parse::<u32>() {
        if let Err(e) = stream.write_all(format!("{}\n", num).as_bytes()).await {
            eprintln!("データの送信に失敗しました: {}", e);
        }
    } else {
        println!("有効な数字を入力してください。");
    }
}

pub async fn ke_send_command(stream: Arc<Mutex<TcpStream>>, input: String) {
    let mut stream = stream.lock().await;
    process_input(&mut stream, &input).await;
}

pub async fn key_communication_loop(stream: TcpStream) {
    let stream = Arc::new(Mutex::new(stream));
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    println!("コマンド（数字）を入力してください。'exit'で終了します:");

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

        let mut stream_locked = stream.lock().await;
        process_input(&mut stream_locked, &input).await;
    }
}
