use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::net::{TcpStream, IpAddr};
use std::io::{Read, Write};
use std::{thread, time};

pub fn discover_server() -> Option<(IpAddr, u16)> {
    let mdns = ServiceDaemon::new().expect("Failed to create mdns daemon");

    // サービスを探索
    let receiver = mdns.browse("_useful_devices._udp.local.").expect("Failed to browse for mDNS services");

    for event in receiver {
        if let ServiceEvent::ServiceResolved(info) = event {
            if let Some(ip) = info.get_addresses().iter().next() {
                return Some((*ip, 5000)); // サーバーはポート5000で待機
            }
        }
    }
    
    None
}

pub fn connect_to_server(ip: IpAddr, port: u16) {
    loop {
        println!("Trying to connect to server at {}:{}", ip, port);

        match TcpStream::connect((ip, port)) {
            Ok(mut stream) => {
                println!("Connected to server at {}:{}", ip, port);

                // サーバーとの通信を維持するループ
                loop {
                    // サーバーにメッセージを送信
                    let command = "Hello, Server!";
                    if let Err(e) = stream.write(command.as_bytes()) {
                        eprintln!("Failed to write to server: {}", e);
                        break; // 書き込み失敗時は接続を切って再接続を試みる
                    } else {
                        println!("Sent message to server: {}", command);
                    }

                    // サーバーからの応答を受け取る
                    let mut buffer = [0; 512];
                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read > 0 {
                                println!("Server response: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                            } else {
                                println!("Server closed the connection.");
                                break; // サーバーが接続を切った場合は再接続
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from server: {}", e);
                            break; // 読み込み失敗時は接続を切って再接続を試みる
                        }
                    }





                    // 10秒ごとにメッセージを送信
                    thread::sleep(time::Duration::from_secs(10));
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
                // 接続失敗時は5秒後に再試行
                thread::sleep(time::Duration::from_secs(5));
            }
        }
    }
}