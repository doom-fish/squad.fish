use gst::glib;
use gst::prelude::*;

mod core_video_buffer;
mod core_video_meta;
mod imp;
mod types;
// The public Rust wrapper type for our element
glib::wrapper! {
    pub struct ScreenCaptureSrc(ObjectSubclass<imp::ScreenCaptureSrc>) @extends gst_base::PushSrc, gst_base::BaseSrc, gst::Element, gst::Object;
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "screencapture",
        gst::Rank::None,
        ScreenCaptureSrc::static_type(),
    )
}
