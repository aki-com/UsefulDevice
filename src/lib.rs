slint::include_modules!();
use slint::{Model, ModelRc};
use std::rc::Rc;
use ud_client::{change_server, get_server};
use std::collections::HashMap;
use std::net::IpAddr;
use std::cell::RefCell;
use tokio;
use tokio::task::{spawn_blocking, LocalSet, spawn_local};

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

async fn device_get_async() -> ModelRc<Device> {
    let device_raw = spawn_blocking(|| get_server()).await.unwrap();
    let device: HashMap<usize, (String, IpAddr)> = device_raw.iter()
        .map(|(&key, (name, ip, _port))| (key, (name.clone(), *ip)))
        .collect();

    let devices = slint::VecModel::from(device.iter().map(|(key, (name, ip))| {
        Device {
            device_name: name.clone().into(),
            IP_address: ip.to_string().into(),
        }
    }).collect::<Vec<_>>());
    slint::ModelRc::new(devices)
}

#[no_mangle]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app).unwrap();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let local_set = LocalSet::new();
    local_set.block_on(&rt, async {
        let ui = Rc::new(RefCell::new(AppWindow::new()?));
        let ui_clone = ui.clone();

        let devices = device_get_async().await;
        ui_clone.borrow().set_devices(devices);

        ui.borrow().on_list_update(move || {
            let ui_clone_2 = ui_clone.clone();
            spawn_local(async move {
                let devices = device_get_async().await;
                ui_clone_2.borrow().set_devices(devices);
            });
        });

        ui.borrow().on_server_connecting(|index| {
            spawn_local(async move {
                let Device { device_name, IP_address } = index;
                let name = device_name.to_string();
                let ip: IpAddr = IP_address.to_string().parse().unwrap();
                let port = 5000;
                println!("Connecting to server: {} {} {}", name, ip, port);
                change_server((name.clone(), ip, port)).await;
            });
        });

        ui.borrow().run()?;
        Ok(())
    })
}