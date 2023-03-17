use std::{thread, time::Duration};

use screencapturekit_sys::sys::{
    content_filter::{InitParams, UnsafeContentFilter},
    shareable_content::UnsafeSCShareableContent,
    stream::{SCStreamHandle, UnsafeSCStream},
    stream_configuration::SCStreamConfiguration,
};

fn main() {
    let display = UnsafeSCShareableContent::get()
        .unwrap()
        .displays()
        .pop()
        .unwrap();
    let params = InitParams::Display(display);
    let filter = UnsafeContentFilter::init(params);

    let config = SCStreamConfiguration {
        width: 100,
        height: 100,
        ..Default::default()
    };
    let handle = SCStreamHandle::init().share();

    let stream = UnsafeSCStream::init(filter, config.into(), handle.clone());
    stream.add_stream_output(handle);
    stream.start_capture();

    thread::sleep(Duration::from_millis(10_000));
}
