mod auth;

async fn auth() -> windows::core::Result<()> {
    if verify_user::verify_user("認証してください").await? {
        println!("認証成功: 保護された処理を実行します");
        // ここに認証後の処理を書く
    } else {
        println!("認証失敗または利用不可");
    }
    Ok(())
}