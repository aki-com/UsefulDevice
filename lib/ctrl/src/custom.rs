use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// アプリケーション設定
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// デバイス名
    pub device_name: String,
    /// ホストアドレス
    pub host: String,
    /// ポート番号
    pub port: u16,
    /// タイムアウト（秒）
    pub timeout_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device_name: "UsefulDevice".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            timeout_secs: 30,
        }
    }
}

impl Config {
    /// ファイルから設定を読み込む
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content)
                .map_err(|e| format!("JSON パース エラー: {}", e)),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    // ファイルが見つからない場合はデフォルト値を返す
                    Ok(Self::default())
                } else {
                    Err(format!("ファイル読み込み エラー: {}", e))
                }
            }
        }
    }

    /// 設定をファイルに保存する
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let path = path.as_ref();
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("JSON シリアライズ エラー: {}", e))?;

        // 親ディレクトリがない場合は作成
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("ディレクトリ作成 エラー: {}", e))?;
        }

        fs::write(path, json)
            .map_err(|e| format!("ファイル書き込み エラー: {}", e))
    }

    /// 設定ディレクトリのパスを取得
    /// プラットフォーム別の標準ディレクトリを返す
    /// - Windows: C:\Users\<username>\AppData\Local\UsefulDevice\
    /// - Linux: ~/.config/useful_device/
    /// - macOS: ~/Library/Application Support/UsefulDevice/
    pub fn config_dir() -> Result<PathBuf, String> {
        dirs::config_dir()
            .ok_or_else(|| "ホームディレクトリを取得できません".to_string())
            .map(|dir| dir.join("UsefulDevice"))
    }

    /// デフォルトのコンフィグファイルパスを取得
    pub fn default_path() -> Result<PathBuf, String> {
        Self::config_dir().map(|dir| dir.join("config.json"))
    }

    /// デフォルト場所からコンフィグを読み込む
    pub fn load_default() -> Result<Self, String> {
        Self::load(Self::default_path()?)
    }

    /// デフォルト場所にコンフィグを保存する
    pub fn save_default(&self) -> Result<(), String> {
        self.save(Self::default_path()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.device_name, "UsefulDevice");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.json");

        let config = Config {
            device_name: "TestDevice".to_string(),
            host: "192.168.1.1".to_string(),
            port: 9000,
            timeout_secs: 60,
        };

        // 保存
        config.save(&path).unwrap();
        assert!(path.exists());

        // 読み込み
        let loaded = Config::load(&path).unwrap();
        assert_eq!(loaded.device_name, "TestDevice");
        assert_eq!(loaded.port, 9000);
    }

    #[test]
    fn test_load_nonexistent_file_returns_default() {
        let config = Config::load("/nonexistent/path/config.json").unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_default_path() {
        let path = Config::default_path().unwrap();
        assert!(path.ends_with("config.json"));
        assert!(path.to_string_lossy().contains("UsefulDevice"));
    }

    #[test]
    fn test_usage_example() {
        // 1️⃣ デフォルト設定を作成
        let mut config = Config::default();
        
        // 2️⃣ 必要に応じて設定を変更
        config.device_name = "MyAndroidDevice".to_string();
        config.host = "192.168.1.100".to_string();
        config.port = 5555;

        // 3️⃣ 設定をファイルに保存
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        config.save(&config_path).unwrap();

        // 4️⃣ ファイルから設定を読み込む
        let loaded = Config::load(&config_path).unwrap();
        
        // 5️⃣ 読み込んだ設定を確認
        assert_eq!(loaded.device_name, "MyAndroidDevice");
        assert_eq!(loaded.host, "192.168.1.100");
        assert_eq!(loaded.port, 5555);
    }
}
