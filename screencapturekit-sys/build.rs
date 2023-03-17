use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

fn get_output_path() -> PathBuf {
    let build_type = env::var("PROFILE").unwrap();
    workspace_dir().join("target").join(build_type)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Re-runs script if any files in res are change
    let protocol_lib_dir = format!(
        "{}/protocolfix/libprotocol_fix.dylib",
        env::var("CARGO_MANIFEST_DIR")?
    );
    let target_dir = get_output_path();
    println!("cargo:rerun-if-changed=./protocolfix/CMakeLists.txt");
    println!("cargo:rerun-if-changed=./protocolfix/ProtocolFix.h");
    println!("cargo:rerun-if-changed=./protocolfix/ProtocolFix.m");

    Command::new("cmake").args(["protocolfix"]).status()?;
    Command::new("make").args(["-C", "protocolfix"]).status()?;

    println!("cargo:warning=CWD {:?}", target_dir);
    std::fs::create_dir(&target_dir).unwrap_or_default();
    std::fs::copy(protocol_lib_dir, target_dir.join("libprotocol_fix.dylib"))?;

    println!(
        "cargo:rustc-link-search=native={}",
        target_dir.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        target_dir.join("screencapturekit-sys").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=protocol_fix");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=Foundation");

    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    Ok(())
}
