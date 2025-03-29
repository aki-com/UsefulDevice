use discover_connect::discover_server;
use std::collections::HashMap;
use std::net::IpAddr;

mod discover_connect;

pub fn client_test(){
    println!("client_test");
}

pub fn get_server() -> HashMap<usize, (String, IpAddr)> {
    let servers: HashMap<usize, (String, IpAddr, u16)> = discover_server();
    servers.iter()
        .map(|(&key, (name, ip, _port))| { // port は無視
            (key, (name.clone(), *ip))
        })
        .collect()
}

pub fn connect_to_server(ip: IpAddr, port: u16){
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