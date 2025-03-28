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

use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = Rc::new(RefCell::new(AppWindow::new()?));
    /*ui.on_show_settings(|| {
        let dialog = Rc::new(device_search::new().unwrap());
        
        let dialog_clone = dialog.clone(); // Clone the Rc pointer
        dialog.on_list_update(move || {
            dialog_clone.set_devices(device_get())
        });
        
        dialog.show().unwrap();
    });*/
    let ui_clone = ui.clone();
    ui.borrow().on_list_update(move || {
        ui_clone.borrow().set_devices(device_get());
    });



    ui.borrow().run()?;

    Ok(())
}