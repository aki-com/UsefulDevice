use ud_ctrl::send_key_combination;
use ud_link::{server_start, connection_accept, register_mdns_service};

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    
    // サーバー起動
    let server = server_start(5000).await?;
    
    // mDNSサービス登録
    match register_mdns_service(5000).await {
        Ok(_) => println!("mDNS service registered successfully"),
        Err(e) => eprintln!("Failed to register mDNS service: {}", e),
    }
    
    // 無限ループで接続待機
    loop {
        // 接続受け入れ
        let mut connection = connection_accept(&server).await?;
        
        // データ受信とコマンド処理のループ
        loop {
            match connection.receive_line().await {
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
