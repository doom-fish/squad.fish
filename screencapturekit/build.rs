use std::{env, process::Command};
fn main() {
    let target_dir = format!("{}/protocolfix", env::var("CARGO_MANIFEST_DIR").unwrap());

    // Re-runs script if any files in res are changed
    println!("cargo:rerun-if-changed=./protocolfix/ProtocolFix.h");
    println!("cargo:rerun-if-changed=./protocolfix/ProtocolFix.m");
    println!("cargo:rerun-if-changed=./protocolfix/install_name.sh");

    Command::new("make")
        .args(["-C", "protocolfix"])
        .status()
        .unwrap();


    println!("cargo:rustc-link-search=native={}", target_dir);
    println!("cargo:rustc-link-lib=static=ProtocolFix");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}
