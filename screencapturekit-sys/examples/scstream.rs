use std::{thread, time::Duration};

use screencapturekit_sys::{
    content_filter::{UnsafeContentFilter, UnsafeInitParams::Display},
    shareable_content::UnsafeSCShareableContent,
    stream::UnsafeSCStream,
    stream_configuration::UnsafeStreamConfiguration,
    stream_error_handler::UnsafeSCStreamError,
    stream_output_handler::{CMSampleBuffer, UnsafeSCStreamOutput},
};

#[repr(C)]
struct TestHandler {}
impl UnsafeSCStreamError for TestHandler {
    fn handle_error(&self) {
        eprintln!("ERROR!");
    }
}
impl UnsafeSCStreamOutput for TestHandler {
    fn got_sample(&self, sample: CMSampleBuffer) {
        eprintln!("SAMPPLE!: {:?}", sample);
    }
}
fn main() {
    let display = UnsafeSCShareableContent::get()
        .unwrap()
        .displays()
        .pop()
        .unwrap();
    let filter = UnsafeContentFilter::init(Display(display));

    let config = UnsafeStreamConfiguration {
        width: 100,
        height: 100,
        ..Default::default()
    };

    let stream = UnsafeSCStream::init(filter, config.into(), TestHandler {});
    stream.add_stream_output(TestHandler {});
    stream.start_capture();

    thread::sleep(Duration::from_millis(10_000));
}
