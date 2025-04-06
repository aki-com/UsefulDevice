use winapi::um::winuser::VK_LWIN;
use windows_volume_control::AudioController;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;
use tokio::sync::MutexGuard;

use std::mem;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, INPUT_TYPE, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_CONTROL, VK_ESCAPE, VK_MENU, VK_SHIFT
};

pub fn set_volume(value: f32){
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
}

pub fn send_key_combination(keys: &[VIRTUAL_KEY]) {
    let len = keys.len() * 2;
    let mut inputs = vec![INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: unsafe { std::mem::zeroed() },
    }; len];

    unsafe {
        // キーを押す
        for (i, &key) in keys.iter().enumerate() {
            let ki = KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0), // 押下
                time: 0,
                dwExtraInfo: 0,
            };
            std::ptr::write(&mut inputs[i].Anonymous as *mut _ as *mut KEYBDINPUT, ki);
            println!("Key pressed: {:?}", key); // デバッグログ
        }

        // キーを離す（逆順）
        for (i, &key) in keys.iter().rev().enumerate() {
            let ki = KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP, // 解放
                time: 0,
                dwExtraInfo: 0,
            };
            std::ptr::write(&mut inputs[keys.len() + i].Anonymous as *mut _ as *mut KEYBDINPUT, ki);
            println!("Key released: {:?}", key); // デバッグログ
        }

        // すべての入力を一度に送信
        let result = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        if result == 0 {
            eprintln!("Failed to send input");
        } else {
            println!("Input sent successfully");
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(200)); // ここで少し待機
}

fn send_ctrl_c() {
    send_key_combination(&[VK_CONTROL, VIRTUAL_KEY(0x43)]);
}

fn send_ctrl_v() {
    send_key_combination(&[VK_CONTROL, VIRTUAL_KEY(0x56)]);
}

fn send_ctrl_a() {
    send_key_combination(&[VK_CONTROL, VIRTUAL_KEY(0x41)]);
}

/*fn send_alt_tab() {
    send_key_combination(&[VK_MENU, VIRTUAL_KEY(0x09)]);
}*/

fn send_ctrl_shift_esc() {
    send_key_combination(&[VK_CONTROL, VK_SHIFT, VK_ESCAPE]);
}

fn send_windows_e() {
    send_key_combination(&[VIRTUAL_KEY(0x5B), VIRTUAL_KEY(0x45)]);
}

fn send_prtsc() {
    send_key_combination(&[VIRTUAL_KEY(0x2C)]);
}

fn send_ctrl_s() {
    send_key_combination(&[VK_CONTROL, VIRTUAL_KEY(0x53)]);
}

fn send_ctrl_p() {
    send_key_combination(&[VK_CONTROL, VIRTUAL_KEY(0x50)]);
}

fn send_win_i() {
    send_key_combination(&[VIRTUAL_KEY(0x5B), VIRTUAL_KEY(0x49)]);
}

fn send_mute() {
    send_key_combination(&[VIRTUAL_KEY(0xAD)]); // ミュートキー
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
                            set_volume(volume_value);
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