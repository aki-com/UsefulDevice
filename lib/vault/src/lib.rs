use reqwest::{Client, multipart};
use serde::Deserialize;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct LoginResponse {
    accessToken: String,
    refreshToken: String,
    // 他のフィールドも必要に応じて
}

pub struct VaultToken {
    base_url: String,
    client: Client,
    access_token: Option<String>,
}

impl VaultToken {
    pub fn new(base_url: &str) -> Self {
        VaultToken {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
            access_token: None,
        }
    }

    /// ログインしてトークンを内部保存
    pub async fn login(&mut self, user_id: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/auth/login", self.base_url);
        let params = serde_json::json!({
            "userID": user_id,
            "password": password,
        });

        let resp = self.client.post(&url)
            .json(&params)
            .send()
            .await?;

        let resp = resp.error_for_status()?;
        let body: LoginResponse = resp.json().await?;
        self.access_token = Some(body.accessToken);
        Ok(())
    }

    /// ファイル一覧取得（オプションでパス指定）
    pub async fn list_files(&self, path: Option<&str>) -> Result<serde_json::Value, Box<dyn Error>> {
        let url = if let Some(p) = path {
            format!("{}/list?path={}", self.base_url, p)
        } else {
            format!("{}/list", self.base_url)
        };

        let resp = self.client.get(&url)
            .bearer_auth(self.access_token.as_ref().expect("Not logged in"))
            .send()
            .await?;

        let resp = resp.error_for_status()?;
        let json = resp.json().await?;
        Ok(json)
    }

    /// ファイルダウンロード
    pub async fn download_file<P: AsRef<Path>>(&self, filename: &str, path: Option<&str>, out_path: P) -> Result<(), Box<dyn Error>> {
        let mut url = format!("{}/download?filename={}", self.base_url, filename);
        if let Some(p) = path {
            url = format!("{}&path={}", url, p);
        }

        let resp = self.client.get(&url)
            .bearer_auth(self.access_token.as_ref().expect("Not logged in"))
            .send()
            .await?;

        let resp = resp.error_for_status()?;
        let bytes = resp.bytes().await?;
        tokio::fs::write(out_path, bytes).await?;
        Ok(())
    }

    /// ファイルアップロード
    pub async fn upload_file<P: AsRef<Path>>(&self, file_path: P, path: Option<&str>) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/upload", self.base_url);

        // Work with an owned PathBuf to avoid moving/borrowing issues
        let path_buf = file_path.as_ref().to_path_buf();
        let file_name: String = path_buf
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file name"))?;

        let file = tokio::fs::read(&path_buf).await?;
        let part = multipart::Part::bytes(file).file_name(file_name);
        let mut form = multipart::Form::new().part("file", part);

        if let Some(p) = path {
            form = form.text("path", p.to_string());
        }

        let resp = self.client.post(&url)
            .bearer_auth(self.access_token.as_ref().expect("Not logged in"))
            .multipart(form)
            .send()
            .await?;

    let _resp = resp.error_for_status()?;
        Ok(())
    }
}

