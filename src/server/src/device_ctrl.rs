use winapi::um::winuser::VK_LWIN;
use windows_volume_control::AudioController;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;
use tokio::sync::MutexGuard;
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use std::thread;
use tokio::time::Duration;

use std::mem;


/*pub fn set_volume(value: f32){
    unsafe {
                
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        let test = controller.get_all_session_names();

        /*println!("{:?}",test);
        let discord_session = controller.get_session_by_name("Discord".to_string());
        println!("{:?}",discord_session.unwrap().getVolume());
        discord_session.unwrap().setVolume(0.5);*/
        println!("{:?}",test);
        let master_volume = controller.get_session_by_name("master".to_string());
        println!("{:?}",master_volume.unwrap().getVolume());
        master_volume.unwrap().setVolume(value);
        println!("音量を{}に設定しました", value);
    }
}*/
//enigo使用バージョン
pub fn send_key_combination(keys: &[Key]) {
    let mut enigo = Enigo::new(&Default::default()).unwrap();

    // キーを押す
    for key in keys {
        enigo.key(*key, Press);
        println!("Key pressed: {:?}", key);
    }

    // 少し待つ（必要に応じて調整）
    thread::sleep(Duration::from_millis(50));

    // キーを離す（逆順）
    for key in keys.iter().rev() {
        enigo.key(*key, Release);
        println!("Key released: {:?}", key);
    }

    thread::sleep(Duration::from_millis(200));
}

fn send_ctrl_c() {
    send_key_combination(&[Key::Control, Key::Unicode('c')]);
}

fn send_ctrl_v() {
    send_key_combination(&[Key::Control, Key::Unicode('v')]);
}

fn send_ctrl_a() {
    send_key_combination(&[Key::Control, Key::Unicode('a')]);
}

fn send_ctrl_shift_esc() {
    send_key_combination(&[Key::Control, Key::Shift, Key::Escape]);
}

fn send_windows_e() {
    send_key_combination(&[Key::Meta, Key::Unicode('e')]);
}

fn send_prtsc() {
    send_key_combination(&[Key::PrintScr]);
}

fn send_ctrl_s() {
    send_key_combination(&[Key::Control, Key::Unicode('s')]);
}

fn send_ctrl_p() {
    send_key_combination(&[Key::Control, Key::Unicode('p')]);
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
                    _ if received.starts_with("volume ") => {
                        let volume_str = &received[7..]; // "volume "の後の部分を取得
                        if let Ok(volume_value) = volume_str.trim().parse::<f32>() {
                            //set_volume(volume_value);
                            let _ = stream.write_all(b"Volume adjusted\n").await;
                        } else {
                            let _ = stream.write_all(b"Invalid volume value\n").await;
                        }
                    }

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