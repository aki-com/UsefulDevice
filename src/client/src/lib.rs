use discover_connect::discover_server;
use std::collections::HashMap;
use std::net::IpAddr;


mod discover_connect;
mod send_cmdID;

pub fn client_test(){
    println!("client_test");
}

pub fn get_server() -> HashMap<usize, (String, IpAddr, u16)> {
    discover_server()

}

pub fn chenge_server(server_map: (String, IpAddr, u16)) {
    let (_name,ip,port) =  server_map .clone();
    if let Some(stream) = discover_connect::connect_to_server(ip, port) {
        send_cmdID::communication_loop(stream);
    } else {
        println!("Could not connect to the server.");
    }

}