/// Windows Hello / PIN 認証を実行する関数
/// 認証に成功したら true を返す
#[cfg(target_os = "windows")]
use windows::{
    core::Result,
    Security::Credentials::UI::{
        UserConsentVerifier, 
        UserConsentVerificationResult, 
        UserConsentVerifierAvailability,
    },
};

pub async fn verify_user(message: &str) -> Result<bool> {
    // Windows Hello の利用可否を確認
    let availability = UserConsentVerifier::CheckAvailabilityAsync()?.get()?;
    if availability != UserConsentVerifierAvailability::Available {
        eprintln!("Windows Hello が利用できません。");
        return Ok(false);
    }

    // 認証ダイアログを表示
    let message_hstring = HSTRING::from(message);
    let result = UserConsentVerifier::RequestVerificationAsync(&message_hstring)?.get()?;

    match result {
        UserConsentVerificationResult::Verified => Ok(true),
        _ => Ok(false),
    }
}

#[cfg(target_os = "android")]
// Androidでは未実装のため、ダミー関数を用意
use jni::objects::{JClass, JObject};
use jni::JNIEnv;

pub fn start_auth(env: &JNIEnv, activity: JObject) {
    let class = env.find_class("com/rustlibs/authentication/Authentication").unwrap();
    env.call_static_method(
        class,
        "authenticate",
        "(Landroid/app/Activity;)V",
        &[activity.into()]
    ).unwrap();
}

// JNIからのコールバック
#[no_mangle]
pub extern "system" fn Java_com_rustlibs_authentication_Authentication_nativeOnAuthResult(
    env: JNIEnv,
    _class: JClass,
    success: bool
) {
    if success {
        println!("認証成功");
    } else {
        println!("認証失敗");
    }
}