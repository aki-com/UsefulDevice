use windows_volume_control::AudioController;
use std::io::{Read, Write};
use std::net::TcpStream;

use std::mem;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, KEYBDINPUT, SendInput, KEYEVENTF_KEYUP, VK_CONTROL, VIRTUAL_KEY, VK_MENU,
    INPUT_TYPE, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS
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
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            };
            std::ptr::write(&mut inputs[i].Anonymous as *mut _ as *mut KEYBDINPUT, ki);
        }

        // キーを離す（逆順）
        for (i, &key) in keys.iter().rev().enumerate() {
            let ki = KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            };
            std::ptr::write(&mut inputs[keys.len() + i].Anonymous as *mut _ as *mut KEYBDINPUT, ki);
        }

        // すべての入力を一度に送信
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
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

fn send_alt_tab() {
    send_key_combination(&[VK_MENU, VIRTUAL_KEY(0x09)]);
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
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
                            let _ = stream.write(b"Volume adjusted\n");
                        } else {
                            let _ = stream.write(b"Invalid volume value\n");
                        }
                    }

                    "1" => {
                        println!("Command 1: Ctrl+C");
                        //ここにCtrl+Cの処理を書く
                        send_ctrl_c();
                    }
                    "2" => {
                        println!("Command 2: Ctrl+V");
                        //ここにCtrl+Vの処理を書く
                        send_ctrl_v();
                    }
                    "3" => {
                        println!("Command 3: Performing shutdown procedure");
                        let _ = stream.write(b"Shutting down device...\n");
                        // breakなどで接続を閉じる
                        break;
                    }
                    "4" => {
                        println!("Command 4: Ctrl+A");
                        //ここにCtrl+Aの処理を書く
                        send_ctrl_a();
                    }
                    "5" => {
                        println!("Command 5: Alt+Tab");
                        //ここにAlt+Tabの処理を書く
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        send_alt_tab();
                        //1️秒待つAlt+Tabを送信
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        println!("Command 5: Alt+Tab");
                        send_alt_tab();
                    }
                    _ => {
                        println!("Unknown command received");
                        let _ = stream.write(b"Unknown command\n");
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