use gst::glib;
use gst::prelude::*;

mod imp;
// The public Rust wrapper type for our element
glib::wrapper! {
    pub struct Screencast(ObjectSubclass<imp::Screencast>) @extends gst::Element, gst::Object;
}

// Registers the type for our element, and then registers in GStreamer under
// the name "gstscreen" for being able to instantiate it via e.g.
// gst::ElementFactory::make().
pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "gstscreen",
        gst::Rank::Primary,
        Screencast::static_type(),
    )
}
