fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
    
    // macOS用の認証ライブラリをリンク
    #[cfg(target_os = "macos")]
{
        println!("cargo:rustc-link-arg=lib/all/auth/src/mac/libauthenticate.a");
    }
}
