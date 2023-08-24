use std::{ptr, sync::Arc};

use gst::{
    ffi::{gst_memory_init, GstMemory, GST_MEMORY_FLAG_READONLY},
    glib::{
        ffi::{gboolean, gpointer},
        translate::{IntoGlib, ToGlibPtr},
    },
    Allocator, Memory,
};
use screencapturekit::sc_sys::rc::Object;

use super::allocator::APPLE_VIDEO_ALLOCATOR_NAME;

type CVPixelBufferRef = *mut Object;
#[derive(Default, Debug)]
#[repr(C)]
pub struct AppleVideoMemory {
    pub(crate) mem: Option<gst::ffi::GstMemory>,
    pub(crate) pxbuf: Option<CVPixelBufferRef>,
    pub(crate) plane: usize,
}

extern "C" {
    fn CVPixelBufferIsPlanar(pxbuf: CVPixelBufferRef) -> bool;
    fn CVPixelBufferGetBaseAddressOfPlane(pxbuf: CVPixelBufferRef, plane: usize) -> gpointer;
    fn CVPixelBufferGetBaseAddress(pxbuf: CVPixelBufferRef) -> gpointer;
    fn CVPixelBufferLockBaseAddress(pxbuf: CVPixelBufferRef, lock_flags: u64) -> i32;
    fn CVPixelBufferUnlockBaseAddress(pxbuf: CVPixelBufferRef, lock_flags: u64) -> i32;
}

const CVPIXEL_BUFFER_LOCK_READ_ONLY: u64 = 0x00000001;
const CVRETURN_SUCCESS: i32 = 0;

pub(crate) unsafe extern "C" fn mem_map(
    mem: *mut GstMemory,
    _size: usize,
    _flags: u32,
) -> gpointer {
    let gmem = ptr::read(mem as *mut AppleVideoMemory);
    if let Some(pxbuf) = gmem.pxbuf {
        if CVPixelBufferLockBaseAddress(pxbuf, CVPIXEL_BUFFER_LOCK_READ_ONLY) == CVRETURN_SUCCESS {
            return if CVPixelBufferIsPlanar(pxbuf) {
                CVPixelBufferGetBaseAddressOfPlane(pxbuf, gmem.plane)
            } else {
                CVPixelBufferGetBaseAddress(pxbuf)
            };
        }
    }
    ptr::null_mut()
}
pub(crate) unsafe extern "C" fn mem_unmap(mem: *mut GstMemory) {
    let gmem = ptr::read(mem as *mut AppleVideoMemory);
    if let Some(pxbuf) = gmem.pxbuf {
        CVPixelBufferUnlockBaseAddress(pxbuf, CVPIXEL_BUFFER_LOCK_READ_ONLY);
    } else {
    }
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
    pub fn new_wrapped(pxbuf: CVPixelBufferRef, plane: usize, size: usize) -> Memory {
        let allocator = Allocator::find(Some(APPLE_VIDEO_ALLOCATOR_NAME))
            .expect("Should have registerd apple video allocator");

        let mem: &mut AppleVideoMemory = Box::leak(Box::default());

        mem.pxbuf = Some(pxbuf);
        mem.plane = plane;

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
