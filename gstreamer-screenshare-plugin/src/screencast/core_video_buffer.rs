use std::{mem::size_of, ptr};

use glib::{ffi::gpointer, translate::IntoGlib};
use gst::ffi::{
    GstAllocator, GstAllocatorClass, GstMemory, GstMeta, GST_ALLOCATOR_FLAG_CUSTOM_ALLOC,
};

use gst_video::{ffi::GST_VIDEO_MAX_PLANES, VideoInfo};

use super::{
    core_video_memory::{
        GstAppleCoreVideoLockState, GstAppleCoreVideoMemory, GstAppleCoreVideoPixelBuffer,
    },
    types::{CMBlockBufferRef, CMSampleBufferRef, CVImageBufferRef, CVPixelBufferRef},
};

extern "C" {
    fn CVPixelBufferRelease(pixel_buf: CVPixelBufferRef);
    fn CVPixelBufferGetPlaneCount(pixel_buf: CVPixelBufferRef) -> usize;
    fn CVPixelBufferGetBytesPerRowOfPlane(pixel_buf: CVPixelBufferRef, index: usize) -> usize;
    fn CVPixelBufferGetHeightOfPlane(pixel_buf: CVPixelBufferRef, index: usize) -> usize;
    fn CVPixelBufferIsPlanar(pixel_buf: CVPixelBufferRef) -> u8;
}

pub unsafe fn gst_core_video_wrap_pixel_buffer(info: &VideoInfo, pixel_buf: CVPixelBufferRef) {
    let num_planes = CVPixelBufferGetPlaneCount(pixel_buf);
    let mut stride = [0; GST_VIDEO_MAX_PLANES as usize];
    let mut offset = [0; GST_VIDEO_MAX_PLANES as usize];
    if CVPixelBufferIsPlanar(pixel_buf) == 1 {
        let mut size = 0;
        let mut plane_offset = 0;
        for i in 0..num_planes {
            stride[i] = CVPixelBufferGetBytesPerRowOfPlane(pixel_buf, i);
            size = stride[i] * CVPixelBufferGetHeightOfPlane(pixel_buf, i);
            offset[i] = plane_offset;
            plane_offset += size;

            // create memory
        }
    }
}
