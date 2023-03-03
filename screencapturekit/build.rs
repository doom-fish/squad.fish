use std::{env, process::Command};
use copy_to_output::copy_to_output;
fn main() {


     // Re-runs script if any files in res are changed  
    println!("cargo:rerun-if-changed=protocolfix/ProtocolFix.h");  
    println!("cargo:rerun-if-changed=protocolfix/ProtocolFix.m");  

    Command::new("make")
        .args(["-C", "protocolfix"])
        .status()
        .unwrap();

    copy_to_output("protocolfix/ProtocolFix.framework", &env::var("PROFILE").unwrap()).expect("Could not copy");  
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}
