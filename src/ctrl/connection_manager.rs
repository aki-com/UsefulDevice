use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type SharedConnection = Arc<Mutex<Option<Arc<Mutex<TcpStream>>>>>;

/// TCP接続管理のための構造体
pub struct ConnectionManager {
    current_connection: SharedConnection,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            current_connection: Arc::new(Mutex::new(None)),
        }
    }

    /// 現在の接続状態を取得
    pub fn get_connection(&self) -> SharedConnection {
        Arc::clone(&self.current_connection)
    }

    /// 新しい接続を受け入れるかどうかをチェック
    pub async fn can_accept_connection(&self) -> bool {
        let conn_guard = self.current_connection.lock().await;
        conn_guard.is_none()
    }

    /// 接続を拒否する
    pub async fn reject_connection(mut stream: TcpStream, reason: &str) {
        let _ = stream.write_all(format!("{}\n", reason).as_bytes()).await;
        let _ = stream.shutdown().await;
    }

    /// 新しい接続を受け入れて初期化
    pub async fn accept_connection(&self, stream: TcpStream, welcome_msg: &str) -> Arc<Mutex<TcpStream>> {
        let stream_arc = Arc::new(Mutex::new(stream));
        
        // 接続状態を記録
        let mut conn_guard = self.current_connection.lock().await;
        *conn_guard = Some(Arc::clone(&stream_arc));
        drop(conn_guard);

        // ウェルカムメッセージを送信
        {
            let mut locked_stream = stream_arc.lock().await;
            let _ = locked_stream.write_all(welcome_msg.as_bytes()).await;
        }

        stream_arc
    }

    /// 接続を閉じる
    pub async fn close_connection(&self) {
        let mut conn_guard = self.current_connection.lock().await;
        *conn_guard = None;
    }
}
