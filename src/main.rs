// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::collections::HashMap;
use std::error::Error;
use slint::{ModelRc, SharedString, Weak};
//use ud_server::server_test;
use ud_client::{change_server, get_server,send_command};
use std::rc::Rc;
use std::cell::RefCell;
use std::net::IpAddr;

slint::include_modules!();






async fn device_get() -> ModelRc<Device> {
    let device_raw = get_server().await; // 非同期版のget_server()が必要

    let devices = slint::VecModel::from(
        device_raw.iter().map(|(_, (name, ip, _))| {
            Device {
                device_name: name.clone().into(),
                IP_address: ip.to_string().into(),
            }
        }).collect::<Vec<_>>()
    );

    slint::ModelRc::new(devices)
}


#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new().unwrap();


{
    let ui_weak = ui.as_weak();
    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        // Use spawn_local for tasks that aren't Send
        tokio::task::spawn(async move {
            let new_model = device_get().await;
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_devices(new_model);
            }
        });
    });
}


       // サーバー接続ハンドラ
    {
        ui.on_server_connecting(|index| {
            let Device { device_name, IP_address } = index;
            let name = device_name.to_string();
            let ip: IpAddr = IP_address.to_string().parse().unwrap();
            let port = 5000;

            println!("Connecting to server: {} {} {}", name, ip, port);
            // We can use regular spawn here as this doesn't capture UI
            tokio::spawn(async move {
                change_server((name, ip, port)).await;
            });
        });
    }

    // コマンド送信ハンドラ
    {
        ui.on_cmd_send(|input| {
            let input = input.to_string();
            println!("Sending command: {}", input);
            // We can use regular spawn here as this doesn't capture UI
            tokio::spawn(async move {
                send_command(input).await;
            });
        });
    }

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