fn main() {
    println!(
        "cargo:rustc-link-search=framework={}",
        "ScreenCaptureKit.framework"
    );
    gst_plugin_version_helper::info()
}
