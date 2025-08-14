use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn biometric_callback(success: c_int) {
    let result = success != 0;
    println!("Rust: 認証結果受信 → {}", result);
}

extern "C" {
    fn authenticate_biometric(callback: extern "C" fn(c_int));
}

pub fn start_biometric_auth() {
    unsafe {
        println!("Rust: 生体認証開始要求");
        authenticate_biometric(biometric_callback);
        println!("Rust: 生体認証要求送信完了");
    }
}