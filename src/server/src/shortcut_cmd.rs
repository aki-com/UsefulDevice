use winapi::um::mmeapi::{waveOutSetVolume, waveOutGetVolume};

/// 指定された数値の音量 (0~100) に調整
pub fn adjust_volume(volume: i32) {
    let volume = volume.clamp(0, 100); // 0~100の範囲に制限
    let new_volume = (volume as u32 * 0xFFFF / 100) as u32; // 0~100 を 0x0000~0xFFFF に変換

    unsafe {
        waveOutSetVolume(std::ptr::null_mut(), (new_volume << 16) | new_volume); // 左右のチャンネルに適用
    }

    println!("Volume set to {}%", volume);
}

pub fn parse_volume_command(command: &str) -> Option<i32> {
    if let Some(start) = command.find("Vad(") {
        if let Some(end) = command.find(")") {
            let num_str = &command[start + 4..end]; // "Vad(" の後ろから ")" までを取得
            return num_str.parse::<i32>().ok();
        }
    }
    None
}
