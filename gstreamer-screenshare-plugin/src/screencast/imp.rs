#![allow(dead_code)]
use std::sync::mpsc::SyncSender;
use std::sync::Mutex;

use gst::subclass::prelude::*;
use gst::{error_msg, glib, Caps, ClockTime};
use gst_base::subclass::base_src::CreateSuccess;
use gst_base::subclass::prelude::*;

use gst_video::VideoFormat;
use once_cell::sync::Lazy;
use screencapturekit::cm_sample_buffer::CMSampleBuffer;
use screencapturekit::sc_content_filter::SCContentFilter;
use screencapturekit::sc_error_handler::StreamErrorHandler;
use screencapturekit::sc_output_handler::{SCStreamOutputType, StreamOutput};
use screencapturekit::sc_shareable_content::SCShareableContent;
use screencapturekit::sc_stream::SCStream;
use screencapturekit::sc_stream_configuration::{PixelFormat, SCStreamConfiguration};
static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "screencapture",
        gst::DebugColorFlags::empty(),
        Some("GStreamer Screencapture Kit"),
    )
});

pub struct ScreenCaptureSrc {
    state: Mutex<State>,
}
pub struct State {
    latency: Option<ClockTime>,
    last_sampling: Option<ClockTime>,
    count: u32,
    stage: Stage,
    receiver: Option<std::sync::mpsc::Receiver<CMSampleBuffer>>,
    stream: Option<SCStream>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            latency: ClockTime::NONE,
            last_sampling: ClockTime::NONE,
            count: Default::default(),
            stage: Default::default(),
            receiver: Default::default(),
            stream: Default::default(),
        }
    }
}

#[derive(Default)]
pub enum Stage {
    Started,
    Stopping,
    #[default]
    Stopped,
}

struct StreamProducer {
    sender: SyncSender<CMSampleBuffer>,
}

#[glib::object_subclass]
impl ObjectSubclass for ScreenCaptureSrc {
    const NAME: &'static str = "GstScreenCaptureKitSrc";
    type Type = super::ScreenCaptureSrc;
    type ParentType = gst_base::PushSrc;

    fn new() -> Self {
        let mut content = SCShareableContent::current();
        let display = content.displays.pop().unwrap();
        let config = SCStreamConfiguration::from_size(display.width, display.height, false);
        let filter = SCContentFilter::new(
            screencapturekit::sc_content_filter::InitParams::Display(display),
        );
        let (sender, receiver) = std::sync::mpsc::sync_channel(2);
        let sp = StreamProducer { sender };
        let mut stream = SCStream::new(filter, config, StreamErr {});
        stream.add_output(sp);
        let state = State {
            stream: Some(stream),
            receiver: Some(receiver),
            ..Default::default()
        };

        Self {
            state: Mutex::new(state),
        }
    }
}

impl StreamOutput for StreamProducer {
    fn did_output_sample_buffer(
        &self,
        sample_buffer: CMSampleBuffer,
        _of_type: SCStreamOutputType,
    ) {
        let _ = self.sender.send(sample_buffer).map_err(|_| {
            error_msg!(
                gst::ResourceError::Failed,
                ("Failed to send Samplebuffer through sync channel")
            )
        });
    }
}
struct StreamErr;
impl StreamErrorHandler for StreamErr {
    fn on_error(&self) {
        todo!()
    }
}

impl GstObjectImpl for ScreenCaptureSrc {}

impl ObjectImpl for ScreenCaptureSrc {}

impl ElementImpl for ScreenCaptureSrc {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "ScreenCaptureKit Source",
                "Source/ScreenCapture",
                "Generates media from ScreenCaptureKit",
                "Per Johansson <per@doom.fish>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            let caps = gst_video::video_make_raw_caps(&[VideoFormat::Nv12]).build();
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &caps,
            )
            .unwrap();

            vec![src_pad_template]
        });
        PAD_TEMPLATES.as_ref()
    }

    fn change_state(
        &self,
        transition: gst::StateChange,
    ) -> Result<gst::StateChangeSuccess, gst::StateChangeError> {
        self.parent_change_state(transition)
    }
}

fn into_video_format(pixel_format: PixelFormat) -> VideoFormat {
    match pixel_format {
        PixelFormat::ARGB8888 => VideoFormat::Bgra,
        PixelFormat::ARGB2101010 => VideoFormat::Gbra10le,
        PixelFormat::YCbCr420v => VideoFormat::Nv12,
        PixelFormat::YCbCr420f => VideoFormat::Nv12,
    }
}
impl BaseSrcImpl for ScreenCaptureSrc {
    fn decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        gst::info!(CAT, imp: self, "ALLOC {:?}", query);
        self.parent_decide_allocation(query)
    }
    fn start(&self) -> Result<(), gst::ErrorMessage> {
        let state = self.state.lock().map_err(|err| {
            error_msg!(
                gst::CoreError::StateChange,
                ("failed to receive fds: {}", err)
            )
        })?;

        let stream = state.stream.as_ref().ok_or(error_msg!(
            gst::CoreError::StateChange,
            ["Could not get stream"]
        ))?;

        stream.start_capture();
        gst::info!(CAT, imp: self, "Started");
        Ok(())
    }
    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        let state = self.state.lock().map_err(|err| {
            error_msg!(
                gst::CoreError::StateChange,
                ("failed to receive fds: {}", err)
            )
        })?;

        let stream = state.stream.as_ref().ok_or(error_msg!(
            gst::CoreError::StateChange,
            ["Could not get stream"]
        ))?;

        stream.stop_capture();
        gst::info!(CAT, imp: self, "Stopped");
        Ok(())
    }
    fn is_seekable(&self) -> bool {
        false
    }

    fn query(&self, query: &mut gst::QueryRef) -> bool {
        return if let gst::QueryViewMut::Latency(ref mut q) = query.view_mut() {
            let state = self.state.lock().expect("Should be able to aquire lock");
            q.set(
                true,
                state.latency.unwrap_or(ClockTime::ZERO),
                state.latency,
            );
            true
        } else {
            false
        };
    }

    fn caps(&self, _filter: Option<&gst::Caps>) -> Option<gst::Caps> {
        let result = Caps::new_empty();
        gst::info!(CAT, imp: self, "CAPS");
        Some(result)
    }

    fn set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        // let info = VideoInfo::from_caps(caps).unwrap();

        gst::info!(CAT, imp: self, "CONFIG for caps {}", caps);
        Ok(())
    }

    fn fixate(&self, caps: gst::Caps) -> gst::Caps {
        gst::info!(CAT, imp: self, "FIXATE for caps {}", caps);
        self.parent_fixate(caps)
    }

    fn unlock(&self) -> Result<(), gst::ErrorMessage> {
        // [bufQueueLock lock];
        //   stopRequest = YES;
        //   [bufQueueLock unlockWithCondition:HAS_BUFFER_OR_STOP_REQUEST];
        //
        //   [permissionCond lock];
        //   permissionStopRequest = YES;
        //   [permissionCond broadcast];
        //   [permissionCond unlock];
        //
        //   return YES;
        //
        Ok(())
    }

    fn unlock_stop(&self) -> Result<(), gst::ErrorMessage> {
        //   [bufQueueLock lock];
        // stopRequest = NO;
        // [bufQueueLock unlockWithCondition:([bufQueue count] == 0) ? NO_BUFFERS : HAS_BUFFER_OR_STOP_REQUEST];
        //
        // [permissionCond lock];
        // permissionStopRequest = NO;
        // [permissionCond unlock];
        //
        // return YES;
        //

        Ok(())
    }
}

impl PushSrcImpl for ScreenCaptureSrc {
    fn create(
        &self,
        _buffer: Option<&mut gst::BufferRef>,
    ) -> Result<CreateSuccess, gst::FlowError> {
        gst::info!(CAT, imp: self, "CREATE CALLED");
        let state = self.state.lock().unwrap();
        let sample = state.receiver.as_ref().unwrap().recv().unwrap();
        gst::info!(CAT, imp: self, "GOT SAMPLE {:?}", sample);

        // CMSampleBufferRef sbuf;
        //  CVImageBufferRef image_buf;
        //  CVPixelBufferRef pixel_buf;
        //  size_t cur_width, cur_height;
        //  GstClockTime timestamp, duration;
        //
        //  [bufQueueLock lockWhenCondition:HAS_BUFFER_OR_STOP_REQUEST];
        //  if (stopRequest) {
        //    [bufQueueLock unlock];
        //    return GST_FLOW_FLUSHING;
        //  }
        //
        //  NSDictionary *dic = (NSDictionary *) [bufQueue lastObject];
        //  sbuf = (__bridge CMSampleBufferRef) dic[@"sbuf"];
        //  timestamp = (GstClockTime) [dic[@"timestamp"] longLongValue];
        //  duration = (GstClockTime) [dic[@"duration"] longLongValue];
        //  CFRetain (sbuf);
        //  [bufQueue removeLastObject];
        //  [bufQueueLock unlockWithCondition:
        //      ([bufQueue count] == 0) ? NO_BUFFERS : HAS_BUFFER_OR_STOP_REQUEST];
        //
        //  /* Check output frame size dimensions */
        //  image_buf = CMSampleBufferGetImageBuffer (sbuf);
        //  if (image_buf) {
        //    pixel_buf = (CVPixelBufferRef) image_buf;
        //    cur_width = CVPixelBufferGetWidth (pixel_buf);
        //    cur_height = CVPixelBufferGetHeight (pixel_buf);
        //
        //    if (width != cur_width || height != cur_height) {
        //      /* Set new caps according to current frame dimensions */
        //      GST_WARNING ("Output frame size has changed %dx%d -> %dx%d, updating caps",
        //          width, height, (int)cur_width, (int)cur_height);
        //      width = cur_width;
        //      height = cur_height;
        //      gst_caps_set_simple (caps,
        //        "width", G_TYPE_INT, width,
        //        "height", G_TYPE_INT, height,
        //        NULL);
        //      gst_pad_push_event (GST_BASE_SINK_PAD (baseSrc), gst_event_new_caps (caps));
        //    }
        //  }
        //
        //  *buf = gst_core_media_buffer_new (sbuf, useVideoMeta, textureCache);
        //  if (*buf == NULL) {
        //    CFRelease (sbuf);
        //    return GST_FLOW_ERROR;
        //  }
        //
        //  CFRelease (sbuf);

        //  GST_BUFFER_OFFSET (*buf) = offset++; OFFSET
        //  GST_BUFFER_OFFSET_END (*buf) = GST_BUFFER_OFFSET (*buf) + 1;
        //  GST_BUFFER_TIMESTAMP (*buf) = timestamp. SET_PTS
        //  GST_BUFFER_DURATION (*buf) = duration;
        //
        //  if (doStatsear
        //
        //    [self updateStatistics];
        //
        //  return GST_FLOW_OK;
        //
        //
        // let buf = unsafe {V
        //     let pixelbuffer = sample.sample_buffer;
        //     let b = gst_core_media_buffer_new(pixelbuffer);
        //     gst::Buffer::from_glib_full(b)
        // };

        {
            // let buf = buf.get_mut().unwrap();
            // buf.set_pts(ClockTime::from_nseconds(
            //     sample.presentation_timestamp.value as u64,
            // ));
        };

        // gst::info!(CAT, imp: self, "BUFFER {:?}", buf);
        // Ok(CreateSuccess::NewBuffer(buf))
        Ok(CreateSuccess::FilledBuffer)
    }
}
