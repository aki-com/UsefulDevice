use slint::{ModelRc, Weak};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AppWindow, Device};
use ud_link::{discover_devices, TcpConnection};
use ud_auth::start_auth_async;
use ud_vault::*;


// グローバルな接続状態
static CONNECTION: std::sync::OnceLock<Arc<Mutex<Option<TcpConnection>>>> = std::sync::OnceLock::new();

fn get_connection() -> &'static Arc<Mutex<Option<TcpConnection>>>{
    CONNECTION.get_or_init(|| Arc::new(Mutex::new(None)))
}

pub fn list_update(ui_weak: Weak<AppWindow>) {
    tokio::spawn(async move {
        let devices_result = discover_devices(1.5).await;
        let devices: Vec<Device> = match devices_result {
            Ok(device_list) => device_list.into_iter().map(|device| Device {
                device_name: device.name.into(),
                IP_address: device.addr.ip().to_string().into(),
            }).collect(),
            Err(e) => {
                eprintln!("デバイス検索エラー: {}", e);
                Vec::new()
            }
        };
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let model = ModelRc::new(slint::VecModel::from(devices));
                ui.set_devices(model);
            }
        });
    });
}

pub fn server_connecting(index: crate::Device) {
    let ip_str = index.IP_address.to_string();
    let addr = format!("{}:5000", ip_str);
    
    tokio::spawn(async move {
        match addr.parse() {
            Ok(socket_addr) => {
                match TcpConnection::connect(socket_addr).await {
                    Ok(connection) => {
                        let conn_arc = get_connection();
                        let mut conn_guard = conn_arc.lock().await;
                        *conn_guard = Some(connection);
                        println!("接続完了");
                    },
                    Err(e) => {
                        eprintln!("接続エラー: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("アドレス解析エラー: {}", e);
            }
        }
    });
}

pub fn cmd_send(input: slint::SharedString) {
    let input = input.to_string();
    tokio::spawn(async move {
        let connection = get_connection();
        let mut conn_guard = connection.lock().await;
        if let Some(ref mut connection) = *conn_guard {
            match connection.send_line(&input).await {
                Ok(_) => {
                    println!("コマンド送信完了: {}", input);
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
pub fn auth_sys(ui_weak: Weak<AppWindow>) -> tokio::task::JoinHandle<bool> {
    tokio::spawn(async move {
        let success = start_auth_async().await;
        success
    })
}

pub fn storage_connect(ui_weak: Weak<AppWindow>){
    tokio::spawn(async move {
        let mut client = VaultToken::new("https://app.nitmcr.f5.si");

        // 2. ログイン（userID, passwordは適宜置き換え）
        client.login("user123", "password123").await;

        // 3. ファイル一覧取得（ルートフォルダの場合は None）
        let files = client.list_files(None).await;

        // 4. ファイル一覧を表示
        println!("Files list: {:#?}", files);
    });
}
