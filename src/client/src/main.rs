mod discover_connect;
//cd src/client; cargo run

fn main() {   
    println!("Searching for servers...");
    
    let servers = discover_connect::discover_server(); // すべてのサーバーを取得

    if servers.is_empty() {
        println!("No servers found.");
        return;
    }

    if let Some((ip, port)) = discover_connect::select_server(&servers) {
        println!("Connecting to server at {}:{}", ip, port);
        discover_connect::connect_to_server(ip, port);
        println!("Client connected to server.");
    } else {
        println!("No server selected.");
    }

    // shortcut_cmd::send_vad_command();

    println!("Client finished execution.");
}
