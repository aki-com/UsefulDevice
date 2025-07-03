#![allow(dead_code)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::MutexGuard;
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard
};
use std::thread;
use tokio::time::Duration;

//enigo使用バージョン

const CTRL: Key = if cfg!(target_os = "macos") {
    Key::Meta
} else {
    Key::Control
};

fn string_to_key(input: &str) -> Option<Key> {
    let lower = input.to_lowercase();
    match lower.as_str() {
        "ctrl" | "control" => Some(Key::Control),
        "shift" => Some(Key::Shift),
        "alt" => Some(Key::Alt),
        "meta" | "win" => Some(Key::Meta),
        "enter" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "esc" | "escape" => Some(Key::Escape),
        "space" => Some(Key::Space),
        "prtsc" | "printscreen" => Some(Key::PrintScr),
        "mute" => Some(Key::VolumeMute),
        _ => {
            // 1文字だけなら Unicode
            let mut chars = input.chars();
            if let (Some(c), None) = (chars.next(), chars.next()) {
                Some(Key::Unicode(c))
            } else {
                None
            }
        }
    }
}


pub fn send_key_combination(keys:&[&str]) {
    let mut enigo = Enigo::new(&Default::default()).unwrap();

    // キーを押す
    for key_str in keys {
        let key = string_to_key(key_str).unwrap_or_else(|| {
            eprintln!("Unknown key: {}", key_str);
            Key::Unicode(key_str.chars().next().unwrap_or('\0')) // デフォルトはUnicode
        });
        let _ = enigo.key(key, Press);
        println!("Key pressed: {:?}", key);
    }

    // 少し待つ（必要に応じて調整）
    thread::sleep(Duration::from_millis(50));

    // キーを離す（逆順）
    for key_str in keys.iter().rev() {
        let key = string_to_key(key_str).unwrap_or_else(|| {
            eprintln!("Unknown key: {}", key_str);
            Key::Unicode(key_str.chars().next().unwrap_or('\0')) // デフォルトはUnicode
        });
        let _ = enigo.key(key, Release);
        println!("Key released: {:?}", key);
    }

    thread::sleep(Duration::from_millis(200));
}
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
                println!("command: {:?}",parts);
                send_key_combination(&parts);

            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}   