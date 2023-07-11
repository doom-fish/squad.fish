use std::fs;

fn main() {
    //    probe_library("gstreamer-1.0").unwrap();
    //  println!(
    //    "cargo:rustc-link-search=native=/usr/local/Cellar/gstreamer/1.22.4/lib/gstreamer-1.0/"
    //);
    // println!("cargo:rustc-link-search=native=/Library/Frameworks/GStreamer.framework/Versions/1.0/lib/gstreamer-1.0");
    // println!("cargo:rustc-link-lib=static=gstapplemedia");
    println!("cargo:rerun-if-changed=src/applemedia");

    cc::Build::new()
        .include("/usr/local/Cellar/gstreamer/1.22.4/include/gstreamer-1.0")
        .include("/usr/local/Cellar/glib/2.76.4/include")
        .include("/usr/local/Cellar/glib/2.76.4/include/glib-2.0")
        .include("/usr/local/Cellar/glib/2.76.4/lib/glib-2.0/include")
        .include("/usr/local/opt/gettext/include")
        .include("/usr/local/Cellar/pcre2/10.42/include")
        .include("/Library/Developer/CommandLineTools/SDKs/MacOSX11.sdk/usr/include/ffi")
        .flag("-L/usr/local/Cellar/gstreamer/1.22.4/lib")
        .flag("-L/usr/local/Cellar/glib/2.76.4/lib")
        .flag("-L/usr/local/opt/gettext/lib")
        .flag("-lgstreamer-1.0")
        .flag("-Wl-rpath,/usr/local/Cellar/gstreamer/1.22.4/lib")
        .flag("-lgobject-2.0")
        .flag("-lglib-2.0")
        .file("src/applemedia/coremediabuffer.h")
        .file("src/applemedia/coremediabuffer.c")
        .file("src/applemedia/corevideobuffer.h")
        .file("src/applemedia/corevideobuffer.c")
        .file("src/applemedia/corevideomemory.h")
        .file("src/applemedia/corevideomemory.c")
        .file("src/applemedia/videotexturecache.h")
        .file("src/applemedia/videotexturecache.m")
        .file("src/applemedia/vtutil.h")
        .file("src/applemedia/vtutil.c")
        .compile("applemedia");
    gst_plugin_version_helper::info()
}
