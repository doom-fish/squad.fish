#![allow(dead_code)]
use std::sync::Mutex;

use gst::subclass::prelude::*;
use gst::{error_msg, glib, loggable_error, Buffer, Caps, ClockTime, Context};
use gst_base::subclass::base_src::CreateSuccess;
use gst_base::subclass::prelude::*;

use gst_gl::*;
use gst_video::{VideoFormat, VideoInfo};
use objc::runtime::Object;
use once_cell::sync::Lazy;
use screencapturekit::sc_stream::SCStream;
use screencapturekit::sc_stream_configuration::{PixelFormat, PIXEL_FORMATS};

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
impl Default for ScreenCaptureSrc {
    fn default() -> Self {
        Self {
            state: Mutex::new(State::default()),
        }
    }
}
pub struct State {
    latency: Option<ClockTime>,
    last_sampling: Option<ClockTime>,
    count: u32,
    stage: Stage,
    stream: Option<SCStream>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            latency: ClockTime::NONE,
            last_sampling: ClockTime::NONE,
            count: Default::default(),
            stage: Default::default(),
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

#[glib::object_subclass]
impl ObjectSubclass for ScreenCaptureSrc {
    const NAME: &'static str = "GstScreenCaptureKitSrc";
    type Type = super::ScreenCaptureSrc;
    type ParentType = gst_base::PushSrc;
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
    fn set_context(&self, context: &Context) {
        // GST_INFO_OBJECT (element, "setting context %s",
        //          gst_context_get_context_type (context));
        //  gst_gl_handle_set_context (element, context,
        //          &ctxh->display, &ctxh->other_context);
        //  GST_ELEMENT_CLASS (parent_class)->set_context (element, context);
        self.parent_set_context(context)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            // let gl_caps = gst_video::VideoCapsBuilder::new()
            //     .features([GL_MEMORY_FEATURE])
            //     .format(VideoFormat::Uyvy)
            //     .field("texture-target", "rectangle")
            //     .build();

            let raw1_caps = gst_video::video_make_raw_caps(&[
                VideoFormat::Nv12,
                VideoFormat::Uyvy,
                VideoFormat::Yuy2,
            ])
            .build();
            let raw2_caps = gst_video::video_make_raw_caps(&[VideoFormat::Bgra]).build();
            let mut full_caps = gst::Caps::new_empty();
            // full_caps.merge(gl_caps);
            full_caps.merge(raw1_caps);
            full_caps.merge(raw2_caps);
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &full_caps,
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
impl ScreenCaptureSrc {
    fn capture_output() {
        // GstClockTime timestamp, duration;
        //
        //  [bufQueueLock lock];
        //
        //  if (stopRequest) {
        //    [bufQueueLock unlock];
        //    return;
        //  }
        //
        //  [self getSampleBuffer:sampleBuffer timestamp:&timestamp duration:&duration];
        //
        //  if (timestamp == GST_CLOCK_TIME_NONE) {
        //    [bufQueueLock unlockWithCondition:([bufQueue count] == 0) ? NO_BUFFERS : HAS_BUFFER_OR_STOP_REQUEST];
        //    return;
        //  }
        //
        //  if ([bufQueue count] == BUFFER_QUEUE_SIZE)
        //    [bufQueue removeLastObject];
        //
        //  [bufQueue insertObject:@{@"sbuf": (__bridge id)sampleBuffer,
        //                           @"timestamp": @(timestamp),
        //                           @"duration": @(duration)}
        //                 atIndex:0];
        //
        //  [bufQueueLock unlockWithCondition:HAS_BUFFER_OR_STOP_REQUEST];
        //
    }
}

type CMSampleBufferRef = *const Object;
type TextureCache = *const Object;

fn into_video_format(pixel_format: PixelFormat) -> VideoFormat {
    match pixel_format {
        PixelFormat::ARGB8888 => VideoFormat::Bgra,
        PixelFormat::ARGB2101010 => VideoFormat::Gbra10le,
        PixelFormat::YCbCr420v => VideoFormat::I420,
        PixelFormat::YCbCr420f => VideoFormat::I420,
    }
}

impl BaseSrcImpl for ScreenCaptureSrc {
    fn decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        //         if let Some(alloc_cap) = query.get().0 {
        //             if let Some(feature) = alloc_cap.features(0) {
        //                 if feature.contains(GL_MEMORY_FEATURE) {
        // let cache_fl = Texture
        //         }
        //             }

        //
        // let features = alloc_caps.;
        // if (gst_caps_features_contains (features, GST_CAPS_FEATURE_MEMORY_GL_MEMORY)) {
        //   GstVideoTextureCacheGL *cache_gl;
        //
        //   cache_gl = textureCache ? GST_VIDEO_TEXTURE_CACHE_GL (textureCache) : NULL;
        //
        //   gst_gl_context_helper_ensure_context (ctxh);
        //   GST_INFO_OBJECT (element, "pushing textures, context %p old context %p",
        //     ctxh->context, cache_gl ? cache_gl->ctx : NULL);
        //   if (cache_gl && cache_gl->ctx != ctxh->context) {
        //     g_object_unref (textureCache);
        //     textureCache = NULL;
        //   }
        //   if (!textureCache)
        //     textureCache = gst_video_texture_cache_gl_new (ctxh->context);
        //   gst_video_texture_cache_set_format (textureCache, format, alloc_caps);

        //     Ok(())
        // }
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
        let mut result = Caps::new_empty();

        for format in PIXEL_FORMATS {
            let cap = Caps::builder("video/x-raw")
                .field("format", into_video_format(format).to_str())
                .field("width", 1000)
                .field("height", 1000)
                .build();
            result.get_mut().unwrap().append(cap);
        }
        Some(result)
    }

    fn set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        let _info = VideoInfo::from_caps(caps);
        let mut _state = self.state.lock().map_err(|e| {
            loggable_error!(gst::CAT_CAPS, format!("Could not aquire state lock: {}", e))
        })?;
        //   height = info.height;
        //   format = info.finfo->format;
        //   latency = gst_util_uint64_scale (GST_SECOND, info.fps_d, info.fps_n);
        //
        //   dispatch_sync (mainQueue, ^{
        //     GST_INFO_OBJECT (element,
        //         "width: %d height: %d format: %s", width, height,
        //         gst_video_format_to_string (format));
        //     int video_format = gst_video_format_to_cvpixelformat (format);
        //     output.videoSettings = [NSDictionary
        //         dictionaryWithObject:[NSNumber numberWithInt:video_format]
        //         forKey:(NSString*)kCVPixelBufferPixelFormatTypeKey];
        //
        //       AVCaptureScreenInput *screenInput = (AVCaptureScreenInput *)input;
        //       screenInput.minFrameDuration = CMTimeMake(info.fps_d, info.fps_n);
        //     gst_caps_replace (&caps, new_caps);
        //     GST_INFO_OBJECT (element, "configured caps %"GST_PTR_FORMAT, caps);
        //
        //     if (![session isRunning]) {
        //       BOOL stopping = NO;
        //
        //       /* If permissions are still pending, wait for a response before
        //        * starting the capture running, or else we'll get black frames */
        //       [permissionCond lock];
        //       if (permissionRequestPending && !permissionStopRequest) {
        //         GST_DEBUG_OBJECT (element, "Waiting for pending device access permission.");
        //         do {
        //           [permissionCond wait];
        //         } while (permissionRequestPending && !permissionStopRequest);
        //       }
        //       stopping = permissionStopRequest;
        //       [permissionCond unlock];
        //
        //       if (!stopping)
        //         [session startRunning];
        //     }
        //
        //     /* Unlock device configuration only after session is started so the session
        //      * won't reset the capture formats */
        //     [device unlockForConfiguration];
        //   });
        //
        //   return success;
        //

        gst::debug!(CAT, imp: self, "Configuring for caps {}", caps);
        Ok(())
    }

    fn fixate(&self, caps: gst::Caps) -> gst::Caps {
        //   GstStructure *structure;
        //
        // new_caps = gst_caps_make_writable (new_caps);
        // new_caps = gst_caps_truncate (new_caps);
        // structure = gst_caps_get_structure (new_caps, 0);
        // /* crank up to 11. This is what the presets do, but we don't use the presets
        //  * in ios >= 7.0 */
        // gst_structure_fixate_field_nearest_int (structure, "height", G_MAXINT);
        // gst_structure_fixate_field_nearest_fraction (structure, "framerate", 30, 1);
        //
        // return gst_caps_fixate (new_caps);
        //
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
        //  CFRelease (sbuf);
        //
        //  GST_BUFFER_OFFSET (*buf) = offset++;
        //  GST_BUFFER_OFFSET_END (*buf) = GST_BUFFER_OFFSET (*buf) + 1;
        //  GST_BUFFER_TIMESTAMP (*buf) = timestamp;
        //  GST_BUFFER_DURATION (*buf) = duration;
        //
        //  if (doStats)
        //    [self updateStatistics];
        //
        //  return GST_FLOW_OK;
        //
        //
        Ok(CreateSuccess::NewBuffer(Buffer::new()))
    }
}
