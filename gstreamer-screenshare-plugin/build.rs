fn main() {
    println!("cargo:rerun-if-changed=src/applemedia");
    let settings = pkg_config::Config::new().probe("gstreamer-1.0").unwrap();

    cc::Build::new()
        .includes(settings.include_paths)
        .flag("-Wno-unused-parameter")
        .flag("-Wno-sign-compare")
        .file("src/applemedia/coremediabuffer.h")
        .file("src/applemedia/coremediabuffer.c")
        .file("src/applemedia/corevideobuffer.h")
        .file("src/applemedia/corevideobuffer.c")
        .file("src/applemedia/corevideomemory.h")
        .file("src/applemedia/corevideomemory.c")
        .compile("applemedia");

    gst_plugin_version_helper::info()
}
