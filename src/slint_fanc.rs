use slint::{ModelRc, SharedString, Weak};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AppWindow, Device};
use ud_client::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Mutex<Option<Client>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
        }
    }
}

async fn get_device() -> Vec<Device> {
    let raw = Client::get_servers().await;

    raw.into_iter().map(|(_, (name, ip, _))| Device {
        device_name: name.into(),
        IP_address: ip.to_string().into(),
    }).collect()
}



pub fn list_update(ui_weak: Weak<AppWindow>) {

    // Use spawn_local for tasks that aren't Send
    tokio::task::spawn(async move {
        let device = get_device().await;
            let _ = slint::invoke_from_event_loop(move || {
            let model = ModelRc::new(slint::VecModel::from(device));
            ui_weak.unwrap().set_devices(model);
        });
    });

}
    
pub fn server_connecting(index: Device, state: &AppState) {
    let Device { device_name, IP_address } = index;
    let name = device_name.to_string();
    let ip: IpAddr = match IP_address.to_string().parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Invalid IP address: {}", IP_address);
            return;
        }
    };
    let port = 5000;

    println!("Connecting to server: {} {} {}", name, ip, port);
    let client_ref = state.client.clone();
    tokio::spawn(async move {
        if let Ok(client) = Client::connect((name, ip, port)).await {
            *client_ref.lock().await = Some(client);
        }
    });
}


pub fn cmd_send(input: SharedString, state: &AppState) {
    let input = input.to_string();
    println!("Sending command: {}", input);
    
    let client_ref = state.client.clone();
    tokio::spawn(async move {
        if let Some(ref mut client) = *client_ref.lock().await {
            let _: Result<String, String> = client.send_command(&input).await;
        } else {
            println!("Not connected to server");
        }
    });
}

