use std::ffi::c_int;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// シンプルなatomic変数で結果を共有
static AUTH_COMPLETED: AtomicBool = AtomicBool::new(false);
static AUTH_RESULT: AtomicI32 = AtomicI32::new(0);

#[no_mangle]
pub extern "C" fn biometric_callback(result: c_int) {
    println!("Rust: Touch IDコールバック受信 result={}", result);
    AUTH_RESULT.store(result, Ordering::Relaxed);
    AUTH_COMPLETED.store(true, Ordering::Relaxed);
}

extern "C" {
    fn authenticate_biometric(callback: extern "C" fn(c_int));
}

pub async fn start_biometric_auth() -> bool {
    println!("Rust: 生体認証開始要求");
    
    // リセット
    AUTH_COMPLETED.store(false, Ordering::Relaxed);
    AUTH_RESULT.store(0, Ordering::Relaxed);
    
    // 認証要求を送信（メインスレッドをブロックしない）
    tokio::task::spawn_blocking(|| {
        println!("Rust: 認証スレッド開始");
        unsafe { authenticate_biometric(biometric_callback) };
        println!("Rust: 生体認証要求送信完了");
    }).await.unwrap();
    
    println!("Rust: Touch ID認証が開始されました、UIは続行します");
    
    // 認証要求だけ送信して即座にtrueを返す
    true
}
