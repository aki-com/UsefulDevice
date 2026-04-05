/// JSON 設定ファイルの使用例
/// 実行: cargo run --example config_usage

use ud_ctrl::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== UsefulDevice 設定ファイルの使用例 ===\n");

    // 📍 1. デフォルトの保存場所を確認
    let config_dir = Config::config_dir()?;
    let config_path = Config::default_path()?;
    println!("📁 設定ディレクトリ: {}", config_dir.display());
    println!("📄 設定ファイルパス: {}\n", config_path.display());

    // 🔄 2. デフォルト場所から設定を読み込む
    println!("読み込み中...");
    let mut config = Config::load_default()?;
    println!("✅ 設定を読み込みました");
    println!("   デバイス名: {}", config.device_name);
    println!("   ホスト: {}", config.host);
    println!("   ポート: {}\n", config.port);

    // ✏️ 3. 設定を編集
    println!("設定を編集します...");
    config.device_name = "MyAndroidDevice".to_string();
    config.host = "192.168.1.100".to_string();
    config.port = 5555;
    config.timeout_secs = 60;

    // 💾 4. 編集した設定を保存
    println!("保存中...");
    config.save_default()?;
    println!("✅ 設定を保存しました\n");

    // 🔄 5. 再度読み込んで確認
    println!("確認のため再度読み込みます...");
    let loaded = Config::load_default()?;
    println!("✅ 設定を読び込みました");
    println!("   デバイス名: {}", loaded.device_name);
    println!("   ホスト: {}", loaded.host);
    println!("   ポート: {}", loaded.port);
    println!("   タイムアウト: {} 秒\n", loaded.timeout_secs);

    // 📋 6. カスタムパスで保存することもできます
    let custom_path = "./my_custom_config.json";
    println!("カスタムパスで保存: {}", custom_path);
    config.save(custom_path)?;
    println!("✅ 保存完了");

    // 7. カスタムパスから読み込む
    let custom_loaded = Config::load(custom_path)?;
    println!("✅ {} から読み込み完了", custom_path);
    println!("   ポート: {}\n", custom_loaded.port);

    println!("=== すべての操作が完了しました！ ===");
    Ok(())
}
