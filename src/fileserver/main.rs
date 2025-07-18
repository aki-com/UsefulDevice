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
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .use_rustls_tls()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    match client
        .post("https://10.28.224.80:8080/upload")
        .multipart(form)
        .send()
        .await
    {
        Ok(res) => {
            println!("Status: {}", res.status());
            let body = res.text().await?;
            println!("Response: {}", body);
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            if e.is_connect() {
                eprintln!("Connection error - check if HTTPS server is running");
            } else if e.is_request() {
                eprintln!("Request error - check URL and server configuration");
            }
        }
    }

    Ok(())
}