use std::io::{self, Write, Read};
use std::net::TcpStream;

pub fn communication_loop(mut stream: TcpStream) {
    loop {
        println!("Enter a number to send to the server:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();
        if input == "exit" {
            println!("Exiting...");
            break;
        }

        if let Ok(_num) = input.parse::<u32>() {
            // 数字送信
            if let Err(e) = stream.write(format!("{}\n", input).as_bytes()) {
                eprintln!("Failed to send data: {}", e);
                break;
            }

            // 応答を受信
            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        println!("Server response: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    } else {
                        println!("Server closed connection.");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from server: {}", e);
                    break;
                }
            }
        } else {
            println!("Please enter a valid number or type 'exit' to quit.");
        }
    }
}
