// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod slint_fanc;
use slint_fanc::{cmd_send, list_update, server_connecting, AppState};

use std::error::Error;


slint::include_modules!();



#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let ui = AppWindow::new().unwrap();
    let ui_weak = ui.as_weak();
    ui.set_platform(std::env::consts::OS.into());

    
    
    let app_state = AppState::new();

    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        list_update(ui_weak);
    }); 

    ui.on_server_connecting({
        let state = app_state.clone();
        move |index| {
            server_connecting(index, &state);
        }
    });
    
    ui.on_cmd_send({
        let state = app_state.clone();
        move |input| {
            cmd_send(input, &state);
        }
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