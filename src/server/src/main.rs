mod server;

mod device_ctrl;
// cd src/server; cargo run


mod audio_ctrl;
fn main() {
    match set_volume(0.5) {  // 50% に設定
        Ok(_) => println!("音量を変更しました"),
        Err(e) => eprintln!("エラー: {:?}", e),
    };


    server::start_server();

    audio_ctrl::adjust_volume(30);
    audio_ctrl::parse_volume_command("Vad(75)");
}




