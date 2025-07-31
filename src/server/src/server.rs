use ud_link::{initialize_server, ServerConfig};
use ud_ctrl::{ClientSession, ConnectionManager};
use crate::client_handler::handle_client;
use std::sync::Arc;

pub async fn start_server() {
    // サーバー設定
    let config = ServerConfig::default();

    // サーバーを初期化
    let server_setup = match initialize_server(config).await {
        Ok(setup) => setup,
        Err(e) => {
            eprintln!("Server initialization failed: {}", e);
            return;
        }
    };

    let connection_manager = Arc::new(ConnectionManager::new());
    let listener = server_setup.listener;

    // クライアント接続を待機
    while let Ok((stream, addr)) = listener.accept().await {
        let manager = Arc::clone(&connection_manager);

        // 接続可能かチェック
        if !manager.can_accept_connection().await {
            println!("Rejected connection from {}: already connected", addr);
            ConnectionManager::reject_connection(stream, "Server busy").await;
            continue;
        }

        println!("Connection established with: {}", addr);

        // 接続を受け入れて初期化
        let stream_arc = manager.accept_connection(stream, "Hello from server!").await;

        // クライアントセッション開始
        let session = ClientSession::new(stream_arc, Arc::clone(&manager));
        session.start(|stream| async move {
            handle_client(stream.lock().await).await;
        }).await;
    }

    println!("Server running. Press Ctrl+C to stop.");
}