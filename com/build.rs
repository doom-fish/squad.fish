fn main() {
    let gstreamer = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("gstreamer-1.0")
        .expect("Failed to find and link GStreamer");

    let gstaudio = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("gstreamer-audio-1.0")
        .expect("Failed to find and link gstreamer-audio-1.0");

    gstreamer
        .include_paths
        .iter()
        .chain(gstaudio.include_paths.iter())
        .for_each(|path| {
            println!("cargo:include={}", path.display());
        });
}
