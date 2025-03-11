mod discover_connect;
mod shortcut_cmd;
fn main() {   
    println!("Searching for server...");
    
    if let Some((ip, port)) = discover_connect::discover_server() {
        println!("Found server at {}:{}", ip, port);
        discover_connect::connect_to_server(ip, port);
        println!("Client connected to server.");
    } else {
        println!("Server not found.");
    }
    
    shortcut_cmd::send_vad_command();

    println!("Client finished execution.");
}
