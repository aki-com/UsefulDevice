mod discover_connect;
mod send_cmdID;
//cd src/client;cargo run
fn main() {
    let servers = discover_connect::discover_server();
    if let Some((ip, port)) = discover_connect::select_server(&servers) {
        if let Some(stream) = discover_connect::connect_to_server(ip, port) {
            send_cmdID::communication_loop(stream);
        } else {
            println!("Could not connect to the server.");
        }
    } else {
        println!("No server selected.");
    }
}