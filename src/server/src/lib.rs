mod device_ctrl;
use enigo::{Enigo, Key};
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

pub fn server_test(){
    println!("server_test");
}

pub fn cmd_send(keys: &[Key]){
    device_ctrl::send_key_combination(keys);
}





