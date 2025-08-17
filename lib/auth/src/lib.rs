#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub use ios::*;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;

pub async fn start_auth() -> bool {
    #[cfg(target_os = "macos")]
    {
        mac::auth().await
    }
    
    #[cfg(target_os = "windows")]
    {
        windows::auth("アプリケーションの認証を行ってください").await.unwrap_or(false)
    }
    
    #[cfg(target_os = "ios")]
    {
        false // TODO
    }
    
    #[cfg(target_os = "android")]
    {
        false // TODO
    }
}

pub async fn start_auth_async() -> bool {
    #[cfg(target_os = "macos")]
    {
        mac::auth().await
    }
    
    #[cfg(target_os = "windows")]
    {
        windows::auth("アプリケーションの認証を行ってください").await.unwrap_or(false)
    }
    
    #[cfg(target_os = "ios")]
    {
        false // TODO
    }
    
    #[cfg(target_os = "android")]
    {
        false // TODO
    }
}