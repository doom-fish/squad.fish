fn main() {
    // Use pkg_config to find and link GStreamer
    let gstreamer = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("gstreamer-1.0")
        .expect("Failed to find and link GStreamer");

    // Use pkg_config to find and link gstaudio-1.0
    let gstaudio = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("gstreamer-audio-1.0")
        .expect("Failed to find and link gstreamer-audio-1.0");

    // Print the paths to the GStreamer and gstaudio-1.0 libraries
    for path in gstreamer
        .include_paths
        .iter()
        .chain(gstaudio.include_paths.iter())
    {
        println!("cargo:include={}", path.display());
        // Add the path to the library to the LD_LIBRARY_PATH environment variable
    }
}
