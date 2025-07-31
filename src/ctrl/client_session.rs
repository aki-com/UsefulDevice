use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use super::{connection_manager::ConnectionManager, periodic_status_sender};

/// クライアントセッション管理
pub struct ClientSession {
    stream: Arc<Mutex<TcpStream>>,
    connection_manager: Arc<ConnectionManager>,
}

impl ClientSession {
    pub fn new(stream: Arc<Mutex<TcpStream>>, connection_manager: Arc<ConnectionManager>) -> Self {
        Self {
            stream,
            connection_manager,
        }
    }

    /// セッションを開始（ステータス送信 + クライアント処理）
    pub async fn start<F, Fut>(self, client_handler: F)
    where
        F: FnOnce(Arc<Mutex<TcpStream>>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let stream_clone = Arc::clone(&self.stream);
        let status_stream = Arc::clone(&self.stream);
        let connection_manager = Arc::clone(&self.connection_manager);

        tokio::spawn(async move {
            // ステータス送信タスクを開始
            tokio::spawn(async move {
                periodic_status_sender(status_stream).await;
            });

            // クライアント処理を実行
            client_handler(stream_clone).await;

            // セッション終了時に接続をクリア
            connection_manager.close_connection().await;
            println!("Client disconnected, ready for new connection");
        });
    }
}
