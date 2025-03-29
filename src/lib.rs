
slint::slint! {
    export component AppWindow inherits Window {
        Text {
            text: "Slint & Android";
        }
    }
}

#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    AppWindow::new().unwrap().run().unwrap();
}