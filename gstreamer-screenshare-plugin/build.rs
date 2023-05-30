fn main() {
  println!("cargo:rustc-link-search=native=/usr/local/Cellar/gstreamer/1.22.3/lib/gstreamer-1.0");
  // println!("cargo:rustc-link-search=native=/Library/Frameworks/GStreamer.framework/Versions/1.0/lib/gstreamer-1.0");
 // println!("cargo:rustc-link-lib=static=gstapplemedia");
  gst_plugin_version_helper::info()
}
