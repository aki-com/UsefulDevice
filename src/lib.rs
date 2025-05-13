#![cfg(target_os = "android")]

slint::include_modules!();
use slint::{Model, ModelRc, SharedString, Weak};
use std::rc::Rc;
use ud_client::{change_server, get_server, send_command};
use std::collections::HashMap;
use std::net::IpAddr;
use std::cell::RefCell;

async fn get_device() -> Vec<Device> {
    let raw = get_server().await;

    raw.into_iter().map(|(_, (name, ip, _))| Device {
        device_name: name.into(),
        IP_address: ip.to_string().into(),
    }).collect()
}

#[no_mangle]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {

    //初期化
    slint::android::init(app).unwrap();
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();
    // 初期デバイスセット

    


    // リスト更新ハンドラ（非同期で更新）
    {
    
    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        // Use spawn_local for tasks that aren't Send
        tokio::task::spawn(async move {
            let device = get_device().await;
            let _ = slint::invoke_from_event_loop(move || {
                let model = ModelRc::new(slint::VecModel::from(device));
                ui_weak.unwrap().set_devices(model);
            });
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
        // 初期デバイスセット
    let devices_model = device_get();
    ui.set_devices(devices_model);
    

    // リスト更新ハンドラ
    {
        let ui_weak = ui.as_weak();
        ui.on_list_update(move || {
            let ui_weak = ui_weak.clone();
            // Use spawn_local for tasks that aren't Send
            tokio::task::spawn_local(async move {
                let new_model = device_get();
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_devices(new_model);
                }
            });
        });
    }

    // Server connecting handler
    {
        ui.on_server_connecting(|index| {
            let Device { device_name, IP_address } = index;
            let name = device_name.to_string();
            let ip: IpAddr = IP_address.to_string().parse().unwrap();
            let port = 5000;
            
            println!("Connecting to server: {} {} {}", name, ip, port);
            tokio::spawn(async move {
                change_server((name, ip, port)).await;
            });
        });
    }

    // Command sending handler
    {
        ui.on_cmd_send(|input| {
            let input = input.to_string();
            println!("Sending command: {}", input);
            tokio::spawn(async move {
                send_command(input).await;
            });
        });
    }

    ui.run()?;

    Ok(())
}