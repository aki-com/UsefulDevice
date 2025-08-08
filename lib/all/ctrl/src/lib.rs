use tokio::net::TcpStream;

// 内部モジュール（非公開）
mod commands;
mod keyboard;

// 公開API

/// コマンドを送信
pub async fn send_command(stream: &mut TcpStream, input: &str) -> Result<String, String> {
    commands::send_command(stream, input).await
}

/// キーの組み合わせを送信
pub fn send_key_combination(keys: &[&str]) -> Result<(), String> {
    keyboard::send_key_combination(keys)
}