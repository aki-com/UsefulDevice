use reqwest::{Client, multipart};
use anyhow::Result;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let file_content = fs::read("foo.txt").await?;
    let file_part = multipart::Part::bytes(file_content)
        .file_name("foo.txt")
        .mime_str("text/plain")?;
    
    let form = multipart::Form::new()
        .part("file", file_part);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)  // ← ここで検証スキップ
        .build()?;

    let res = client
        .post("https://10.28.224.80:8080/upload")
        .multipart(form)
        .send()
        .await?;

    println!("Status: {}", res.status());
    Ok(())
}