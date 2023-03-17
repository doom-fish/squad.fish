extern crate screencapturekit;

use std::{thread, time::Duration};

use objc_foundation::INSObject;
use screencapturekit::sys::{
    content_filter::{InitParams, UnsafeContentFilter},
    shareable_content::UnsafeSCShareableContent,
    stream::{SCStreamHandle, UnsafeSCStream},
    stream_configuration::SCStreamConfiguration,
};
#[link(name = "ProtocolFix", kind = "static")]
extern "C" {}

fn main() {
    let display = UnsafeSCShareableContent::get()
        .unwrap()
        .displays()
        .pop()
        .unwrap();
    let params = InitParams::Display(display);
    let filter = UnsafeContentFilter::init(params);

    let mut config = SCStreamConfiguration::default();
    config.width = 100;
    config.height = 100;
    let handle = SCStreamHandle::new().share();

    let stream = UnsafeSCStream::init(filter, config.into(), handle.clone());
    stream.add_stream_output(handle);
    stream.start_capture();

    thread::sleep(Duration::from_millis(10_000));
}
