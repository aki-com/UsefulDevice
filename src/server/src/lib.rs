mod device_ctrl;


pub fn server_test(){
    println!("server_test");
}

pub fn cmd_send(keys: &[VIRTUAL_KEY]){
    device_ctrl::send_key_combination(keys);
}





