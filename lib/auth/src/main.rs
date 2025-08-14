#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
use mac::*;
use std::{thread, time::Duration};

fn main() {
    println!("生体認証を開始します...");
    start_biometric_auth();

    // 10秒待機して Swift 側のコールバックを受け取る
    thread::sleep(Duration::from_secs(10));
}
