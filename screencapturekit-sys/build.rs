use apple_bindgen::Builder;

use std::env;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // rerurn if the generated source file does not exist
    let out_dir = env::var("OUT_DIR").unwrap() + "/bindings.rs";
    println!("cargo:rerun-if-not-exists={}", out_dir);

    let builder = Builder::with_builtin_config("ScreenCaptureKit", "macosx")?;

    let out = builder.generate()?;
    fs::write(out_dir, out)?;
    Ok(())
}
