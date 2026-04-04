
fn main() {
    #[cfg(target_os = "macos")]
    {
        use std::env;
        use std::path::PathBuf;
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_file = PathBuf::from(manifest_dir).join("src/mac/libauthenticate.a");
        println!("cargo:rustc-link-arg={}", lib_file.display());
    }
}
