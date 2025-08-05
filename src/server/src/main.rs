use ud_link::{start_tcp_server, accept_connection, receive_data};
use ud_ctrl::send_key_combination;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    
    // サーバー起動
    let (_ip, listener) = start_tcp_server().await?;
    
    // 無限ループで接続待機
    loop {
        // 接続受け入れ
        let mut connection = accept_connection(&listener).await?;
        
        // データ受信とコマンド処理のループ
        loop {
            match receive_data(&mut connection).await {
                Ok(data) => {
                    println!("Received command: {}", data);
                    let parts: Vec<&str> = data.split('+').collect();
                    
                    // キーコマンド実行
                    match send_key_combination(&parts) {
                        Ok(_) => println!("Key combination executed successfully"),
                        Err(e) => eprintln!("Failed to execute key combination: {}", e),
                    }
                }
                Err(e) => {
                    println!("Connection closed: {}", e);
                    break;
                }
            }
        }
    }
}
