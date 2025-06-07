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

pub fn send_key_combination(keys: &[Key]) {
    let mut enigo = Enigo::new(&Default::default()).unwrap();

    // キーを押す
    for key in keys {
        let _ = enigo.key(*key, Press);
        println!("Key pressed: {:?}", key);
    }

    // 少し待つ（必要に応じて調整）
    thread::sleep(Duration::from_millis(50));

    // キーを離す（逆順）
    for key in keys.iter().rev() {
        let _ = enigo.key(*key, Release);
        println!("Key released: {:?}", key);
    }

    thread::sleep(Duration::from_millis(200));
}

fn send_ctrl_c() {
    send_key_combination(&[CTRL, Key::Unicode('c')]);
}

fn send_ctrl_v() {
    send_key_combination(&[CTRL, Key::Unicode('v')]);
}

fn send_ctrl_a() {
    send_key_combination(&[CTRL, Key::Unicode('a')]);
}

fn send_ctrl_shift_esc() {
    send_key_combination(&[CTRL, Key::Shift, Key::Escape]);
}

fn send_windows_e() {
    send_key_combination(&[Key::Meta, Key::Unicode('e')]);
}
#[cfg(target_os = "windows")]
fn send_prtsc() {
    send_key_combination(&[Key::PrintScr]);
}
#[cfg(target_os = "macos")]
fn send_prtsc() {
    send_key_combination(&[Key::Meta, Key::Shift,Key::Unicode('3')]); // MacではCommand + 3でスクリーンショット
}

fn send_ctrl_s() {
    send_key_combination(&[CTRL, Key::Unicode('s')]);
}

fn send_ctrl_p() {
    send_key_combination(&[CTRL, Key::Unicode('p')]);
}

fn send_win_i() {
    send_key_combination(&[Key::Meta, Key::Unicode('i')]);
}

fn send_mute() {
    send_key_combination(&[Key::VolumeMute]); 
}

fn send_volume_up() {
    send_key_combination(&[Key::VolumeUp]);
}

fn send_volume_down() {
    send_key_combination(&[Key::VolumeDown]);
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
                println!("Received command: {}", received);

                match received.as_str() {
                    
                    "1" => {
                        println!("Command 1: ctrl_shift_esc");
                        send_ctrl_shift_esc();
                    }
                    "2" => {
                        println!("Command 2: Explorer");
                        send_windows_e();
                    }
                    "3" => {
                        println!("Command 3: Print Screen");
                        send_prtsc();
                        let _ = stream.write_all(b"Print Screen command sent\n").await;
                    }
                    "4" => {
                        println!("Command 4: Ctrl+S");
                        send_ctrl_s();
                    }
                    "5" => {
                        println!("Command 5: Ctrl+P");
                        send_ctrl_p();
                    }
                    "6" => {
                        println!("Command 6: win+i");
                        send_win_i();
                    }
                    "7" => {
                        println!("Command 7: Ctrl+C");
                        send_ctrl_c();
                    }
                    "8" => {
                        println!("Command 8: Ctrl+V");
                        send_ctrl_v();
                    }
                    "9" => {
                        println!("Command 9: Ctrl+A");
                        send_ctrl_a();
                    }
                    "10" => {
                        println!("Command 10: mute");
                        send_mute();
                    }
                    "11" => {
                        println!("Command 11: volume up");
                        send_volume_up();
                    }
                    "12" => {
                        println!("Command 12: volume down");
                        send_volume_down();
                    }
                    _ => {
                        println!("Unknown command received");
                        let _ = stream.write_all(b"Unknown command\n").await;
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