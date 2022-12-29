fn main() {
    pkg_config::Config::new().probe("gstreamer-1.0").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    tauri_build::build()
}
