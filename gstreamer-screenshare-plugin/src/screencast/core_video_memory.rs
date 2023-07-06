use gst::ffi::GstMeta;
use screencapturekit::sc_stream::CMSampleBuffer;

use super::types::{CMBlockBufferRef, CMSampleBufferRef, CVImageBufferRef, CVPixelBufferRef};

struct GstCoreMediaMeta {
    meta: GstMeta,
    sample_buf: CMSampleBufferRef,
    image_buf: CVImageBufferRef,
    pixel_buf: CVPixelBufferRef,
    block_buf: CMBlockBufferRef,
}

impl GstCoreMediaMeta { 

  fn new() -> Self {
       
    }
}

