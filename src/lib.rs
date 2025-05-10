


slint::include_modules!();
use slint::{Model, ModelRc};
use std::rc::Rc;
use ud_client::{change_server, get_server,send_command};
use std::collections::HashMap;
use std::net::IpAddr;
use std::cell::RefCell;

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


#[cfg(target_os = "android")]
#[no_mangle]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app).unwrap();

    let ui = Rc::new(RefCell::new(AppWindow::new()?));
    let ui_clone = ui.clone();


    ui_clone.borrow().set_devices(device_get());
    ui.borrow().on_list_update(move || {
        ui_clone.borrow().set_devices(device_get());
    });

    ui.borrow().on_server_connecting(|index| {
        let Device{device_name, IP_address} = index;
            let name = device_name.to_string();
            let ip :IpAddr = IP_address.to_string().parse().unwrap();
            let port = 5000;
            println!("Connecting to server: {} {} {}", name, ip, port);
            tokio::spawn(async move {
                println!("Connecting to server: {} {} {}", name, ip, port);
                change_server((name.clone(), ip, port)).await;
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