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