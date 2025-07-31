use slint::{ModelRc, Weak};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;

use crate::{AppWindow, Device};
use ud_link::{discovery::get_devices, connect_to_device, DeviceInfo};
use ud_ctrl::send_command;

// グローバルな接続状態
static CONNECTION: std::sync::OnceLock<Arc<Mutex<Option<TcpStream>>>> = std::sync::OnceLock::new();

fn get_connection() -> &'static Arc<Mutex<Option<TcpStream>>> {
    CONNECTION.get_or_init(|| Arc::new(Mutex::new(None)))
}

pub fn list_update(ui_weak: Weak<AppWindow>) {
    tokio::spawn(async move {
        let device_info = get_devices().await;
        let devices: Vec<Device> = device_info.into_iter().map(|info| Device {
            device_name: info.device_name.into(),
            IP_address: info.ip_address.into(),
        }).collect();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let model = ModelRc::new(slint::VecModel::from(devices));
                ui.set_devices(model);
            }
        });
    });
}

pub fn server_connecting(index: crate::Device) {
    let device_info = DeviceInfo {
        device_name: index.device_name.to_string(),
        ip_address: index.IP_address.to_string(),
        port: 5000,
    };
    
    tokio::spawn(async move {
        match connect_to_device(&device_info).await {
            Ok(stream) => {
                let connection = get_connection();
                let mut conn_guard = connection.lock().await;
                *conn_guard = Some(stream);
                println!("接続完了");
            },
            Err(e) => {
                eprintln!("接続エラー: {}", e);
            }
        }
    });
}

pub fn cmd_send(input: slint::SharedString) {
    let input = input.to_string();
    tokio::spawn(async move {
        let connection = get_connection();
        let mut conn_guard = connection.lock().await;
        if let Some(ref mut stream) = *conn_guard {
            match send_command(stream, &input).await {
                Ok(response) => {
                    println!("レスポンス: {}", response);
                },
                Err(e) => {
                    eprintln!("コマンド送信エラー: {}", e);
                }
            }
        } else {
            eprintln!("接続されていません");
        }
    });
}

