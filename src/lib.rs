#![cfg(any(target_os = "android", target_os = "ios"))]

mod slint_fanc;


use std::error::Error;

use ud_client::{change_server, get_server,send_command};
use slint_fanc::{cmd_send, list_update, server_connecting};

slint::include_modules!();

#[cfg(target_os = "android")]
#[no_mangle]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {

    //初期化
    slint::android::init(app).unwrap();
    let ui = AppWindow::new()?;
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

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn ios_main() {
    // Create a local task set for the UI thread
    let local = tokio::task::LocalSet::new();
    
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
            
        // Run the local tasks on this thread
        rt.block_on(local.run_until(async_main()));
    });
}

#[cfg(target_os = "ios")]
async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
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