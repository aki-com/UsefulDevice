// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::error::Error;
use slint::{Model, ModelRc};
//use ud_server::server_test;
use ud_client::{change_server, get_server};
use tokio::net::TcpStream;
slint::include_modules!();




use std::net::IpAddr;

fn device_get() -> ModelRc<Device> {
    let device_raw = get_server(); //デバイスの取得

    let device: HashMap<usize, (String, IpAddr)> = device_raw.iter().map(|(&key, (name, ip, _port))| { // port は無視
        (key, (name.clone(), *ip))
    })
    .collect();
    let devices = slint::VecModel::from(device.iter().map(|(key, (name, ip))| {
        Device {
            device_name: name.clone().into(),
            IP_address: ip.to_string().into(),
        }
    }).collect::<Vec<_>>());
    slint::ModelRc::new(devices)
}

use std::rc::Rc;
use std::cell::RefCell;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = Rc::new(RefCell::new(AppWindow::new()?));

    let ui_clone = ui.clone();
    ui_clone.borrow().set_devices(device_get());
    ui.borrow().on_list_update(move || {
        ui_clone.borrow().set_devices(device_get());
    });

    ui.borrow().on_server_connecting(|index| {
        let Device { device_name, IP_address } = index;
        let name = device_name.to_string();
        let ip: IpAddr = IP_address.to_string().parse().unwrap();
        let port = 5000;
        println!("Connecting to server: {} {} {}", name, ip, port);
        
        // `tokio::spawn` を使って非同期処理として `change_server` を実行
        tokio::spawn(async move {
            change_server((name.clone(), ip, port)).await;
        });
    });

    ui.borrow().run()?;
    Ok(())
}

/*ui.on_show_settings(|| {
        let dialog = Rc::new(device_search::new().unwrap());
        
        let dialog_clone = dialog.clone(); // Clone the Rc pointer
        dialog.on_list_update(move || {
            dialog_clone.set_devices(device_get())
        });
        
        dialog.show().unwrap();
    });*/