use std::ptr;

use gst::{
    ffi::{gst_memory_init, GstMemory, GST_MEMORY_FLAG_READONLY},
    glib::{
        ffi::{gboolean, gpointer},
        translate::{IntoGlib, ToGlibPtr},
    },
    Allocator, Memory,
};
use screencapturekit::cm_sample_buffer::CMSampleBuffer;

use super::allocator::APPLE_VIDEO_ALLOCATOR_NAME;

#[derive(Debug)]
#[repr(C)]
pub struct AppleVideoMemory {
    pub(crate) mem: Option<gst::ffi::GstMemory>,
    pub(crate) cm_sample_buf: CMSampleBuffer,
    pub(crate) plane_index: u64,
}


pub(crate) unsafe extern "C" fn mem_map(
    mem: *mut GstMemory,
    _size: usize,
    _flags: u32,
) -> gpointer {
    let gmem = ptr::read(mem as *mut AppleVideoMemory);
    let pxbuf = gmem.cm_sample_buf.pixel_buffer;
    if pxbuf.lock() {
        return if pxbuf.is_planar {
            pxbuf.get_base_adress_of_plane(gmem.plane_index)
        } else {
            pxbuf.get_base_adress()
        };
    }
    ptr::null_mut()
}
pub(crate) unsafe extern "C" fn mem_unmap(mem: *mut GstMemory) {
    let gmem = ptr::read(mem as *mut AppleVideoMemory);
    gmem.cm_sample_buf.pixel_buffer.unlock();
}
pub(crate) unsafe extern "C" fn mem_share(
    _mem: *mut GstMemory,
    _offset: isize,
    _size: isize,
) -> *mut GstMemory {
    ptr::null_mut()
}
pub(crate) unsafe extern "C" fn mem_is_span(
    _mem1: *mut GstMemory,
    _mem2: *mut GstMemory,
    _offset: *mut usize,
) -> gboolean {
    false.into_glib()
}

impl AppleVideoMemory {
    #[allow(dead_code)]
    pub fn new_wrapped(cm_sample_buf: CMSampleBuffer, plane_index: u64, size: usize) -> Memory {
        let allocator = Allocator::find(Some(APPLE_VIDEO_ALLOCATOR_NAME))
            .expect("Should have registerd apple video allocator");

        let mem = Box::leak(Box::new(AppleVideoMemory {
            cm_sample_buf,
            plane_index,
            mem: None,
        }));

        let mem_ptr = mem as *mut _ as *mut GstMemory;

        unsafe {
            gst_memory_init(
                mem_ptr,
                GST_MEMORY_FLAG_READONLY,
                allocator.to_glib_none().0,
                std::ptr::null_mut(),
                size,
                0,
                0,
                size,
            );
            Memory::from_glib_none(mem_ptr)
        }
    }
}
