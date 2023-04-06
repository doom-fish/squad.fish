// use screencapturekit_sys::{
//     content_filter::{UnsafeContentFilter, UnsafeContentFilterInitParams},
//     shareable_content::UnsafeSCShareableContent,
//     stream::{UnsafeSCStream, UnsafeSCStreamError},
//     stream_configuration::UnsafeStreamConfiguration,
// };
//
fn main() {
    // let display = UnsafeSCShareableContent::get()
    //     .unwrap()
    //     .displays()
    //     .pop()
    //     .unwrap();
    // let params = UnsafeContentFilterInitParams::Display(display);
    // let filter = UnsafeContentFilter::init(params);
    //
    // let config = UnsafeStreamConfiguration {
    //     width: 100,
    //     height: 100,
    //     ..Default::default()
    // };
    // let handle = UnsafeSCStreamError::init();
    //
    // let stream = UnsafeSCStream::init(filter, config.into(), handle.clone());
    // stream.add_stream_output(handle);
    // stream.start_capture();
    //
    // thread::sleep(Duration::from_millis(10_000));
}
