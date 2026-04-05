use tokio::net::TcpStream;

// 内部モジュール（非公開）
mod commands;
mod keyboard;
mod custom;

// 公開API を再エクスポート
pub use custom::Config;

/// コマンドを送信
pub async fn send_command(stream: &mut TcpStream, input: &str) -> Result<String, String> {
    commands::send_command(stream, input).await
}

/// キーの組み合わせを送信
pub fn exe_key(keys: &[&str]) -> Result<(), String> {
    keyboard::exe_key_combination(keys)
}

/// 🔧 デフォルト場所から設定を読み込む
pub fn load_config() -> Result<Config, String> {
    Config::load_default()
}

/// 💾 設定をデフォルト場所に保存する
pub fn save_config(config: &Config) -> Result<(), String> {
    config.save_default()
}

