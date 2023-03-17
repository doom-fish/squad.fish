use std::{thread, time::Duration};

use screencapturekit_sys::{
    content_filter::{UnsafeContentFilter, UnsafeContentFilterInitParams},
    shareable_content::UnsafeSCShareableContent,
    stream::{UnsafeSCStream, UnsafeSCStreamHandle},
    stream_configuration::UnsafeSCStreamConfiguration,
};

fn main() {
    let display = UnsafeSCShareableContent::get()
        .unwrap()
        .displays()
        .pop()
        .unwrap();
    let params = UnsafeContentFilterInitParams::Display(display);
    let filter = UnsafeContentFilter::init(params);

    let config = UnsafeSCStreamConfiguration {
        width: 100,
        height: 100,
        ..Default::default()
    };
    let handle = UnsafeSCStreamHandle::init().share();

    let stream = UnsafeSCStream::init(filter, config.into(), handle.clone());
    stream.add_stream_output(handle);
    stream.start_capture();

    thread::sleep(Duration::from_millis(10_000));
}
