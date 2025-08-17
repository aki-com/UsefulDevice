#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
use mac::*;
use std::{thread, time::Duration};
#[cfg(target_os = "windows")]
use ud_auth::auth;
#[tokio::main]
async fn main() {
    println!("生体認証を開始します...");
    auth("生体認証を実行してください").await;

    // 10秒待機して Swift 側のコールバックを受け取る
    thread::sleep(Duration::from_secs(10));
}
