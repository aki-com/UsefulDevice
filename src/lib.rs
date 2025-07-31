#![cfg(any(target_os = "android", target_os = "ios"))]

mod slint_fanc;
pub mod link;  // 通信レイヤー
pub mod ctrl;  // アプリケーションレイヤー

use std::error::Error;

use ud_client::Client;
use slint_fanc::{cmd_send, list_update, server_connecting};

slint::include_modules!();

#[cfg(target_os = "android")]
#[no_mangle]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) -> Result<(), Box<dyn std::error::Error>> {

    //初期化
    slint::android::init(app).unwrap();
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        list_update(ui_weak);
    });

    ui.on_server_connecting(|index| {
        server_connecting(index);
    });
    ui.on_cmd_send(|input| {
        cmd_send(input);
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
        // 初期デバイスセット

    // リスト更新ハンドラ
    ui.on_list_update(move || {
        let ui_weak = ui_weak.clone();
        list_update(ui_weak);
    });

    ui.on_server_connecting(|index| {
        server_connecting(index);
    });
    ui.on_cmd_send(|input| {
        cmd_send(input);
    });

    ui.run()?;

    Ok(())
}

