/// シンプルな使用例
/// 実行: cargo run --example simple_usage --lib

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ud_ctrl::Config;

    // 設定を読み込む
    let mut config = Config::load_default()?;

    // 設定を編集
    config.port = 8000;

    // 設定を保存
    config.save_default()?;

    println!("✅ {} にポート番号を保存しました", Config::default_path()?.display());
    Ok(())
}
