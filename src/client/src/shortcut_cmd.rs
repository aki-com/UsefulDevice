//imputで文字を受け取り、vad+任意の値に制限する条件をつける

use std::io::{self, Write};
use std::{thread, time};

pub fn send_vad_command() {
    let mut input = String::new();
    //vad+1~100の任意の値に制限する条件をつける
    loop {
        print!("Enter a command (vad+1~100): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.starts_with("vad") {
            let value: u8 = input[3..].parse().unwrap();
            if value >= 1 && value <= 100 {
                println!("Sending VAD command with value: {}", value);
                break;
            } else {
                println!("Invalid value. Please enter a value between 1 and 100.");
            }
        } else {
            println!("Invalid command. Please enter a command starting with 'vad'.");
        }
    }
}
    