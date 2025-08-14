use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard
};
use std::thread;
use std::time::Duration;

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
        "mute" => Some(Key::VolumeMute),
        "volup" | "volumeup" => Some(Key::VolumeUp),
        "voldown" | "volumedown" => Some(Key::VolumeDown),
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

/// キーの組み合わせを送信する
pub fn send_key_combination(keys: &[&str]) -> Result<(), String> {
    let mut enigo = Enigo::new(&Default::default()).map_err(|e| e.to_string())?;

    // キーを押す
    for key_str in keys {
        let key = string_to_key(key_str).unwrap_or_else(|| {
            eprintln!("Unknown key: {}", key_str);
            Key::Unicode(key_str.chars().next().unwrap_or('\0'))
        });
        enigo.key(key, Press).map_err(|e| e.to_string())?;
        println!("Key pressed: {:?}", key);
    }

    // 少し待つ
    thread::sleep(Duration::from_millis(50));

    // キーを離す（逆順）
    for key_str in keys.iter().rev() {
        let key = string_to_key(key_str).unwrap_or_else(|| {
            eprintln!("Unknown key: {}", key_str);
            Key::Unicode(key_str.chars().next().unwrap_or('\0'))
        });
        enigo.key(key, Release).map_err(|e| e.to_string())?;
        println!("Key released: {:?}", key);
    }

    thread::sleep(Duration::from_millis(200));
    Ok(())
}
