// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod slint_fanc;


use std::error::Error;

use ud_client::{change_server, get_server,send_command};
use slint_fanc::{cmd_send, list_update, server_connecting};

slint::include_modules!();



#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    

    let ui = AppWindow::new().unwrap();
    let ui_weak = ui.as_weak();

    
    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        list_update(ui_weak);
    });

    ui.on_server_connecting(|index| {
        server_connecting(index);
    });
    ui.on_cmd_send(|input| {
        cmd_send(input);
    });

    ui.run()?;

    Ok(())
}
/*
ui.on_show_settings(|| {
        let dialog = Rc::new(device_search::new().unwrap());
        
        let dialog_clone = dialog.clone(); // Clone the Rc pointer
        dialog.on_list_update(move || {
            dialog_clone.set_devices(device_get())
        });
        
        dialog.show().unwrap();
    });*/
#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {
    // AndroidやiOSのメイン関数はここに記述
        // AndroidやiOSのUI初期化コードをここに記述
    // 例: slint::android::init(app).unwrap();
}