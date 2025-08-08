// Prevent console window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod slint_fanc;
use slint_fanc::{list_update, server_connecting, cmd_send};

use std::error::Error;

slint::include_modules!();

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let ui = AppWindow::new().unwrap();
    let ui_weak = ui.as_weak();
    ui.set_platform(std::env::consts::OS.into());

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
#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() -> Result<(), Box<dyn Error>> {
    // Android and iOS entry points are defined in their respective modules
    Ok(())
}