mod server;
mod device_ctrl;
// cd src/server; cargo run

#[tokio::main]
async fn main() {
    server::start_server().await;
}