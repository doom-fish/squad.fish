use copy_to_output::copy_to_output;
use std::{env, process::Command};
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", $($tokens)*)
    }
}
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let target_dir = format!("{}/target/{}", env::var("CARGO_MANIFEST_DIR").unwrap(), profile);

    // Re-runs script if any files in res are changed
    println!("cargo:rerun-if-changed=protocolfix/ProtocolFix.h");
    println!("cargo:rerun-if-changed=protocolfix/ProtocolFix.m");

    Command::new("make")
        .args(["-C", "protocolfix"])
        .status()
        .unwrap();

    p!(target_dir);
    copy_to_output("protocolfix/ProtocolFix.framework", profile.as_str()).expect("Could not copy");

    println!("cargo:rustc-link-search=framework={}", target_dir);
    println!("cargo:rustc-link-lib=framework=ProtocolFix");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}
