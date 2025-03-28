use std::net::TcpStream;
use std::io::{self, Write, Read};
use std::str::FromStr;


pub fn communication_loop(mut stream: TcpStream) {
    loop {
        println!("コマンド（数字）を入力するか、音量を変更するには 'volume <値>' と入力してください。または 'exit' で終了します:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力の読み込みに失敗しました");

        let input = input.trim();
        if input == "exit" {
            println!("終了します...");
            break;
        }

        // 音量調整コマンドのチェック
        if let Some(volume_command) = input.strip_prefix("volume ") {
            if let Ok(volume_change) = volume_command.trim().parse::<f32>() {
                // 音量調整コマンドを送信
                if let Err(e) = stream.write(format!("volume {}\n", volume_change).as_bytes()) {
                    eprintln!("データの送信に失敗しました: {}", e);
                    break;
                }

                // サーバーからの応答処理（必要に応じて）
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            println!("サーバーの応答: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                        } else {
                            println!("サーバーが接続を閉じました。");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("サーバーからの読み込みに失敗しました: {}", e);
                        break;
                    }
                }
            } else {
                println!("有効な音量調整値を入力してください。");
            }
        } else if let Ok(num) = input.parse::<u32>() {
            // 数字コマンドを送信
            if let Err(e) = stream.write(format!("{}\n", num).as_bytes()) {
                eprintln!("データの送信に失敗しました: {}", e);
                break;
            }

            // サーバーからの応答処理
            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        println!("サーバーの応答: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    } else {
                        println!("サーバーが接続を閉じました。");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("サーバーからの読み込みに失敗しました: {}", e);
                    break;
                }
            }
        } else {
            println!("有効な数字、音量コマンド、または 'exit' で終了します。");
        }
    }
}