// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use slint::{Model, ModelRc};
use ud_server::server_test;
use ud_client::{client_test, get_server};

slint::include_modules!();






fn device_get() -> ModelRc<Device> {
    let device_raw = get_server(); //デバイスの取得
    let devices = slint::VecModel::from(device_raw.iter().map(|(key, (name, ip))| {
        Device {
            device_name: name.clone().into(),
            IP_address: ip.to_string().into(),
        }
    }).collect::<Vec<_>>());
    slint::ModelRc::new(devices)
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    ui.on_show_settings(move|| {
        let dialog = device_search::new().unwrap();
        dialog.set_devices(device_get());
        dialog.show().unwrap();


    });
    
   // ui.set_name(slint::ModelRc::new(devices));
//    ui.global::<App_Data>().set_devices(slint::ModelRc::new(devices));

    //ui.on_○○でslintのイベントを登録する

    

    ui.run()?;

    Ok(())
}