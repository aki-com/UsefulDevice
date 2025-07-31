mod server;
mod client_handler;

use server::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}