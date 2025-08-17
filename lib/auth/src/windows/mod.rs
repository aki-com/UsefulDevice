/// Windows Hello / PIN 認証を実行する関数
/// 認証に成功したら true を返す
use windows::{
    core::{Result, HSTRING},
    Security::Credentials::UI::{
        UserConsentVerifier, 
        UserConsentVerificationResult, 
        UserConsentVerifierAvailability,
    },
};

pub async fn auth(message: &str) -> Result<bool> {
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

