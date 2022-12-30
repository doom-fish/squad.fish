use cocoa::{
    appkit::{NSView, NSViewHeightSizable, NSViewWidthSizable},
    base::{id, Nil},
    foundation::{NSPoint, NSSize},
};
use gst::prelude::*;
use gst_video::prelude::*;

#[macro_use]
extern crate objc;

use tauri::{plugin::Plugin, Invoke, Manager, Runtime, Window};
pub struct GstreamerPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> GstreamerPlugin<R> {
    // you can add configuration fields here,
    // see https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![]),
        }
    }
}

impl<R: Runtime> Plugin<R> for GstreamerPlugin<R> {
    fn name(&self) -> &'static str {
        "gstreamer"
    }

    /// Callback invoked when the Window is created.
    fn created(&mut self, window: Window<R>) {
        gst::init().unwrap();

        let pipeline = gst::Pipeline::default();
        let src = gst::ElementFactory::make("videotestsrc").build().unwrap();
        let sink = gst::ElementFactory::make("glimagesink").build().unwrap();
        pipeline.add_many(&[&src, &sink]).unwrap();
        src.link(&sink).unwrap();
        let video_overlay = sink
            .clone()
            .dynamic_cast::<gst_video::VideoOverlay>()
            .unwrap()
            .downgrade();

        let bus = pipeline.bus().unwrap();

        bus.set_sync_handler(move |_, msg| {
            let video_overlay = match video_overlay.upgrade() {
                Some(video_overlay) => video_overlay,
                None => return gst::BusSyncReply::Pass,
            };

            if let gst::MessageView::Element(_) = msg.view() {
                if gst_video::is_video_overlay_prepare_window_handle_message(msg) {
                    #[cfg(target_os = "macos")]
                    unsafe {
                        let gstreamer_view_class = class!(NSView);
                        let gst_view: id = msg_send![gstreamer_view_class, alloc];
                        let window = window.ns_window().unwrap() as id;
                        let content_view: id = msg_send![window, contentView];
                        let s: id = msg_send![content_view, superview];
                        let _: () = msg_send![gst_view, init];

                        gst_view.setFrameOrigin(NSPoint::new(0., 0.));
                        gst_view.setFrameSize(content_view.frame().size);
                        gst_view.setAutoresizingMask_(NSViewHeightSizable | NSViewWidthSizable);
                        let _: () = msg_send![
                            s,
                            addSubview: gst_view
                            positioned: -1
                            relativeTo: content_view
                        ];

                        video_overlay.set_window_handle(gst_view as usize);
                    }
                }
            }
            gst::BusSyncReply::Pass
        });

        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");
    }
}
