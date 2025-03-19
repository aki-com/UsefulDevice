use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::net::{TcpStream, IpAddr};
use std::io::{Read, Write};
use std::{thread, time, collections::HashMap};
use std::io;
use std::time::Duration;

pub fn discover_servers() -> HashMap<usize, (IpAddr, u16)> {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");
    let receiver = mdns.browse("_useful_devices._udp.local.").expect("Failed to browse for mDNS services");

    let mut servers = HashMap::new();
    let mut index = 1;

    println!("Searching for servers...");

    // 5秒間探索する
    let timeout = Duration::from_secs(5);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < timeout {
        if let Ok(event) = receiver.recv_timeout(Duration::from_secs(1)) { // 1秒ごとにチェック
            if let ServiceEvent::ServiceResolved(info) = event {
                if let Some(ip) = info.get_addresses().iter().next() {
                    if !servers.values().any(|(existing_ip, _)| existing_ip == ip) {
                        servers.insert(index, (*ip, 5000)); // ポート5000
                        println!("{}: {}:{}", index, ip, 5000);
                        index += 1;
                    }
                }
            }
        } else {
            break; // 受信がタイムアウトしたら終了
        }
    }

    servers
}

pub fn select_server(servers: &HashMap<usize, (IpAddr, u16)>) -> Option<(IpAddr, u16)> {
    if servers.is_empty() {
        return None;
    }

    loop {
        println!("Enter the number of the server you want to connect to:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if let Ok(choice) = input.trim().parse::<usize>() {
            if let Some(server) = servers.get(&choice) {
                return Some(*server);
            }
        }

        println!("Invalid selection. Try again.");
    }
}

pub fn connect_to_server(ip: IpAddr, port: u16) {
    loop {
        println!("Trying to connect to server at {}:{}", ip, port);

        match TcpStream::connect((ip, port)) {
            Ok(mut stream) => {
                println!("Connected to server at {}:{}", ip, port);

                loop {
                    let command = "Hello, Server!";
                    if let Err(e) = stream.write(command.as_bytes()) {
                        eprintln!("Failed to write to server: {}", e);
                        break;
                    } else {
                        println!("Sent message to server: {}", command);
                    }

                    let mut buffer = [0; 512];
                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read > 0 {
                                println!("Server response: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                            } else {
                                println!("Server closed the connection.");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from server: {}", e);
                            break;
                        }
                    }

                    thread::sleep(time::Duration::from_secs(10));
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
                thread::sleep(time::Duration::from_secs(5));
            }
        }
    }
}