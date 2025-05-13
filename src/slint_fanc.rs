use slint::{ModelRc, SharedString, Weak};
use std::rc::Rc;
use std::cell::RefCell;
use std::net::IpAddr;



use crate::{AppWindow, Device};
use ud_client::{change_server, get_server,send_command};

async fn get_device() -> Vec<Device> {
    let raw = get_server().await;

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
    
pub fn server_connecting(index: Device) {
    let Device { device_name, IP_address } = index;
    let name = device_name.to_string();
    let ip: IpAddr = IP_address.to_string().parse().unwrap();
    let port = 5000;

    println!("Connecting to server: {} {} {}", name, ip, port);
    // We can use regular spawn here as this doesn't capture UI
    tokio::spawn(async move {
        change_server((name, ip, port)).await;
    });
}


pub fn cmd_send(input: SharedString) {
    let input = input.to_string();
    println!("Sending command: {}", input);
    // We can use regular spawn here as this doesn't capture UI
    tokio::spawn(async move {
        send_command(input).await;
    });
}

