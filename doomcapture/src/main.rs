#![allow(dead_code)]
use std::{
    process,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
};

use gst::traits::{ElementExt, GstObjectExt};
use screencapturekit::{
    sc_content_filter::InitParams::Display,
    sc_content_filter::SCContentFilter,
    sc_error_handler::StreamErrorHandler,
    sc_output_handler::StreamOutput,
    sc_shareable_content::SCShareableContent,
    sc_stream::{CMSampleBuffer, SCStream},
    sc_stream_configuration::SCStreamConfiguration,
};

struct DoomCaptureOutput {
    tx: SyncSender<CMSampleBuffer>,
}
struct DoomCaptureError;
impl StreamOutput for DoomCaptureOutput {
    fn stream_output(&self, sample: CMSampleBuffer) {
        self.tx.send(sample).expect("Should be able to send!");
    }
}

impl StreamErrorHandler for DoomCaptureError {
    fn on_error(&self) {
        eprintln!("ERROR!")
    }
}

struct DoomCapture {
    stream: SCStream,
    pub rx: Receiver<CMSampleBuffer>,
}

impl DoomCapture {
    fn start(&self) {
        self.stream.start_capture();
    }
    fn new() -> Self {
        let mut content = SCShareableContent::current();
        let display = content.displays.pop().unwrap();
        println!("Display id: {:?}", display.display_id);
        println!("Display width: {:?}", display.width);
        println!("Display height: {:?}", display.height);
        let filter = SCContentFilter::new(Display(display.clone()));

        let config = SCStreamConfiguration::from_size(display.width, display.height, false);

        let (tx, rx) = sync_channel(1);
        let mut stream = SCStream::new(filter, config, DoomCaptureError {});
        stream.add_output(DoomCaptureOutput { tx });

        Self { stream, rx }
    }
}

fn new_pipeline() {
    gst::init().unwrap();

    // Let GStreamer create a pipeline from the parsed launch syntax on the cli.
    // In comparison to the launch_glib_main example, this is using the advanced launch syntax
    // parsing API of GStreamer. The function returns a Result, handing us the pipeline if
    // parsing and creating succeeded, and hands us detailed error information if something
    // went wrong. The error is passed as gst::ParseError. In this example, we separately
    // handle the NoSuchElement error, that GStreamer uses to notify us about elements
    // used within the launch syntax, that are not available (not installed).
    // Especially GUIs should probably handle this case, to tell users that they need to
    // install the corresponding gstreamer plugins.
    let mut context = gst::ParseContext::new();
    let pipeline = match gst::parse_launch_full(
        "videotestsrc num-buffers=100 ! video/x-raw,width=1280,height=720 ! vtenc_h265_hw realtime=true quality=1 ! h265parse ! mp4mux ! filesink location=out.mp4",
        Some(&mut context),
        gst::ParseFlags::empty(),
    ) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                println!("Missing element(s): {:?}", context.missing_elements());
            } else {
                println!("Failed to parse pipeline: {err}");
            }

            process::exit(-1)
        }
    };

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    //    let capture = DoomCapture::new();
    new_pipeline();
    // capture.start();
    // loop {
    //     capture.rx.recv().unwrap();
    //     println!("GOT SAMPLE");
    // }
}
