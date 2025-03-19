// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use ud_server::server_test;
use ud_client::client_test;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    server_test();
    client_test(); 
    let devices = slint::VecModel::from(vec![
        Device {
            device_name: "Device1".into(),
            IP_address: "192.168.1.1".into(),
        },
        Device {
            device_name: "Device2".into(),
            IP_address: "192.168.1.2".into(),
        },
    ]);
    ui.on_show_settings(|| {
        let dialog = device_search::new().unwrap();
        dialog.show().unwrap();
    });
    ui.set_name(slint::ModelRc::new(devices));

    

    //ui.on_○○でslintのイベントを登録する

    

    ui.run()?;

    Ok(())
}