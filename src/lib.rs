
#![cfg(target_os = "android")]

slint::include_modules!();
use slint::{Model, ModelRc};
use std::rc::Rc;
use ud_client::{change_server, get_server,send_command};
use std::collections::HashMap;
use std::net::IpAddr;
use std::cell::RefCell;

async fn device_get() -> ModelRc<Device> {
    let device_raw = get_server().await; // デバイスの取得

    let device: HashMap<usize, (String, IpAddr)> = device_raw.iter().map(|(&key, (name, ip, _port))| {
        (key, (name.clone(), *ip))
    }).collect();

    let devices = slint::VecModel::from(device.iter().map(|(_key, (name, ip))| {
        Device {
            device_name: name.clone().into(),
            IP_address: ip.to_string().into(),
        }
    }).collect::<Vec<_>>());

    slint::ModelRc::new(devices)
}



#[no_mangle]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app).unwrap();

    let ui = Rc::new(RefCell::new(AppWindow::new()?));
    let ui_clone = ui.clone();

    // 初期デバイスセット
    let devices_model = device_get().await;
    ui.borrow().set_devices(devices_model);

    // リスト更新ハンドラ（非同期で更新）
    let ui_clone2 = ui.clone();
    ui.borrow().on_list_update(move || {
        let ui_clone = ui_clone2.clone();
        tokio::spawn(async move {
            let new_model = device_get().await;
            ui_clone.borrow().set_devices(new_model);
        });
    });

    // サーバー接続ハンドラ
    ui.borrow().on_server_connecting(|index| {
        let Device { device_name, IP_address } = index;
        let name = device_name.to_string();
        let ip: IpAddr = IP_address.to_string().parse().unwrap();
        let port = 5000;

        println!("Connecting to server: {} {} {}", name, ip, port);
        tokio::spawn(async move {
            change_server((name, ip, port)).await;
        });
    });

    // コマンド送信ハンドラ
    ui.borrow().on_cmd_send(move |input| {
        let input = input.to_string();
        println!("Sending command: {}", input);
        tokio::spawn(async move {
            send_command(input).await;
        });
    });

    ui.borrow().run()?;

    Ok(())
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn ios_main() {
    std::thread::spawn(|| {
        let _ = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async_main());
    });
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = Rc::new(RefCell::new(AppWindow::new()?));
    let ui_clone = ui.clone();

    // 初期デバイスセット
    let devices_model = device_get().await;
    ui.borrow().set_devices(devices_model);

    // リスト更新ハンドラ
    let ui_clone2 = ui.clone();
    ui.borrow().on_list_update(move || {
        let ui_clone = ui_clone2.clone();
        tokio::spawn(async move {
            let new_model = device_get().await;
            ui_clone.borrow().set_devices(new_model);
        });
    });

    ui.borrow().on_server_connecting(|index| {
        let Device {
            device_name,
            IP_address,
        } = index;
        let name = device_name.to_string();
        let ip: IpAddr = IP_address.to_string().parse().unwrap();
        let port = 5000;
        println!("Connecting to server: {} {} {}", name, ip, port);
        tokio::spawn(async move {
            change_server((name, ip, port)).await;
        });
    });

    ui.borrow().on_cmd_send(move |input| {
        let input = input.to_string();
        println!("Sending command: {}", input);
        tokio::spawn(async move {
            send_command(input).await;
        });
    });

    ui.borrow().run()?;

    Ok(())
}