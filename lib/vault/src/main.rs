mod lib;
use lib::VaultToken;


use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. クライアント作成
    let mut client = VaultToken::new("https://app.nitmcr.f5.si");

    // 2. ログイン（userID, passwordは適宜置き換え）
    client.login("user123", "password123").await?;

    // 3. ファイル一覧取得（ルートフォルダの場合は None）
    let files = client.list_files(None).await?;

    // 4. ファイル一覧を表示
    println!("Files list: {:#?}", files);

    Ok(())
}
