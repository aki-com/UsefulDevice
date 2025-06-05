use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

pub async fn receive_status(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("サーバーから状態を受信中...");

    // データを受信
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    if n > 0 {
        let received_data = String::from_utf8_lossy(&buffer[..n]);
        println!("受信したデータ: {}", received_data);
    } else {
        println!("サーバーからのデータがありません");
    }

    Ok(())
}