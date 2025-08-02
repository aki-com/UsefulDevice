#![cfg(any(target_os = "android", target_os = "ios"))]

mod slint_fanc;


use std::error::Error;

use ud_client::Client;
use slint_fanc::{cmd_send, list_update, server_connecting, AppState};

slint::include_modules!();

#[cfg(target_os = "android")]
#[no_mangle]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {

    //初期化
    slint::android::init(app).unwrap();
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();
    let app_state = AppState::new();

    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        list_update(ui_weak);
    });

    let state_clone = app_state.clone();
    ui.on_server_connecting(move |index| {
        server_connecting(index, &state_clone);
    });
    
    let state_clone = app_state.clone();
    ui.on_cmd_send(move |input| {
        cmd_send(input, &state_clone);
    });

    ui.run()?;

    Ok(())
}

#[cfg(target_os = "ios")]
#[no_mangle]
#[tokio::main]
async fn ios_main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();
    let app_state = AppState::new();
    // 初期デバイスセット

    // リスト更新ハンドラ
    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        list_update(ui_weak);
    });

    let state_clone = app_state.clone();
    ui.on_server_connecting(move |index| {
        server_connecting(index, &state_clone);
    });
    
    let state_clone = app_state.clone();
    ui.on_cmd_send(move |input| {
        cmd_send(input, &state_clone);
    });

    ui.run()?;

    Ok(())
}

