/// 設定機能のテストバイナリ
/// 使用例: cargo run --bin test_config

use ud_ctrl::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== UsefulDevice 設定機能テスト ===\n");

    // 1️⃣ デフォルト設定の確認
    println!("📋 デフォルト設定:");
    let default_config = Config::default();
    println!("  デバイス名: {}", default_config.device_name);
    println!("  ホスト: {}", default_config.host);
    println!("  ポート: {}", default_config.port);
    println!("  タイムアウト: {} 秒\n", default_config.timeout_secs);

    // 2️⃣ 保存場所を確認
    println!("📁 ファイル保存場所:");
    let config_path = Config::default_path()?;
    println!("  {}\n", config_path.display());

    // 3️⃣ デフォルト場所から読み込む
    println!("🔄 デフォルト場所から読み込み中...");
    let mut config = Config::load_default()?;
    println!("✅ 読み込み完了");
    println!("  デバイス名: {}", config.device_name);
    println!("  ホスト: {}", config.host);
    println!("  ポート: {}\n", config.port);

    // 4️⃣ 設定を編集
    println!("✏️ 設定を編集します...");
    config.device_name = "TestDevice".to_string();
    config.host = "192.168.1.50".to_string();
    config.port = 9999;
    config.timeout_secs = 120;
    println!("  新しいデバイス名: {}", config.device_name);
    println!("  新しいホスト: {}", config.host);
    println!("  新しいポート: {}\n", config.port);

    // 5️⃣ 編集した設定を保存
    println!("💾 設定をファイルに保存中...");
    config.save_default()?;
    println!("✅ 保存完了\n");

    // 6️⃣ 再度読み込んで確認
    println!("🔄 再度読み込んで検証中...");
    let loaded = Config::load_default()?;
    println!("✅ 読み込み完了");
    
    // 設定が正しく保存・読み込みされたか確認
    assert_eq!(loaded.device_name, config.device_name);
    assert_eq!(loaded.host, config.host);
    assert_eq!(loaded.port, config.port);
    assert_eq!(loaded.timeout_secs, config.timeout_secs);
    println!("  ✓ デバイス名が一致: {}", loaded.device_name);
    println!("  ✓ ホストが一致: {}", loaded.host);
    println!("  ✓ ポートが一致: {}", loaded.port);
    println!("  ✓ タイムアウトが一致: {} 秒\n", loaded.timeout_secs);

    println!("=== すべてのテストが成功しました! ✨ ===\n");

    Ok(())
}
