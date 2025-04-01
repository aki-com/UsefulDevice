slint::include_modules!();
use slint::{Model, ModelRc};
use std::rc::Rc;
use ud_client::{client_test, get_server};

fn device_get() -> ModelRc<Device> {
    let device_raw = get_server(); //デバイスの取得
    let devices = slint::VecModel::from(device_raw.iter().map(|(key, (name, ip))| {
        Device {
            device_name: name.clone().into(),
            IP_address: ip.tostring().into(),
        }
    }).collect::<Vec<>>());
    slint::ModelRc::new(devices)
}


#[no_mangle]
fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    slint::android::init(app).unwrap();

    let ui = Rc::new(RefCell::new(AppWindow::new()?));
    let ui_clone = ui.clone();



    ui.borrow().on_list_update(move || {
        ui_clone.borrow().set_devices(device_get());
        get_server();
    });



    ui.borrow().run()?;

    Ok(())

}