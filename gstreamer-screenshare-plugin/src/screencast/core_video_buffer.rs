use std::{ffi::c_int, mem::size_of, ptr,  borrow::BorrowMut};

use glib::{ffi::gpointer, translate::IntoGlib};
use gst::ffi::{GstAllocator, GstAllocatorClass, GstMemory, gst_memory_flags_get_type, GST_ALLOCATOR_FLAG_CUSTOM_ALLOC};
use gst_video::{ffi::GST_VIDEO_MAX_PLANES, VideoInfo};

use super::types::CVPixelBufferRef;

extern "C" {
    fn CVPixelBufferRelease(pixel_buf: CVPixelBufferRef);
    fn CVPixelBufferGetPlaneCount(pixel_buf: CVPixelBufferRef) -> usize;
    fn CVPixelBufferGetBytesPerRowOfPlane(pixel_buf: CVPixelBufferRef, index: usize) -> usize;
    fn CVPixelBufferGetHeightOfPlane(pixel_buf: CVPixelBufferRef, index: usize) -> usize;
    fn CVPixelBufferIsPlanar(pixel_buf: CVPixelBufferRef) -> u8;
}

#[repr(C)]
enum GstAppleCoreVideoLockState {
    GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED,
    GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READONLY,
    GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READ_WRITE,
}

#[repr(C)]
struct GstAppleCoreVideoPixelBuffer {
    refcount: i32,
    mutex: *mut glib::ffi::GMutex,
    buf: CVPixelBufferRef,
    lock_state: GstAppleCoreVideoLockState,
    lock_count: usize,
}

#[repr(C)]
struct GstAppleCoreVideoMemory {
    mem: GstMemory,
    gpixbuf: *mut GstAppleCoreVideoPixelBuffer,
    plane: usize,
}

fn gst_core_video_wrap_pixel_buffer(info: &VideoInfo, pixel_buf: CVPixelBufferRef) {
    unsafe {
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
    };
}

#[repr(C)]
struct GstAppleCoreVideoAllocatorClass {
    parent_class: GstAllocatorClass,
}

#[repr(C)]
struct GstAppleCoreVideoAllocator {
    parent_instance: GstAllocator,
}

const GST_APPLE_CORE_VIDEO_ALLOCATOR_NAME: * const i8 = b"AppleCoreVideoMemory\0".as_ptr() as *const _;

unsafe fn gst_apple_core_video_pixel_buffer_unref(gpixbuf: *mut GstAppleCoreVideoPixelBuffer) {
    let mut buf: GstAppleCoreVideoPixelBuffer = ptr::read(gpixbuf as *const _);
    if glib::ffi::g_atomic_int_dec_and_test(&buf.refcount as  *const _ as *mut _) == 1 {
        if !matches!(buf.lock_state, GstAppleCoreVideoLockState::GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED) {
             //PANIC
        }
      CVPixelBufferRelease (buf.buf);
      glib::ffi::g_mutex_clear (buf.mutex);
      glib::ffi::g_slice_free1 (size_of::<GstAppleCoreVideoPixelBuffer>(), gpixbuf as gpointer);
    }
}

unsafe extern "C" fn gst_apple_core_video_mem_free(_: *mut GstAllocator, gmem: *mut GstMemory) {
    let memory: GstAppleCoreVideoMemory = ptr::read(gmem as *const _);
    gst_apple_core_video_pixel_buffer_unref(memory.gpixbuf);
    glib::ffi::g_slice_free1(
        size_of::<GstAppleCoreVideoMemory>(),
        &memory as *const _ as gpointer,
    );
}

unsafe extern "C" fn gst_apple_core_video_allocator_class_init(
    klass: *mut GstAppleCoreVideoAllocatorClass,
) {
    let mut allocator: GstAllocatorClass = ptr::read(klass as *const _);
    allocator.alloc = None;
    allocator.free = Some(gst_apple_core_video_mem_free);
}


unsafe extern "C" fn gst_apple_core_video_mem_share (gmem: *mut GstMemory, offset: *mut usize, size: *mut usize)
{

 let mem: GstAppleCoreVideoMemory = ptr::read(gmem as *const _);
 let basemem = ptr::read(gmem);
  /* find the real parent */
  let mut parent = basemem.parent;
  if parent.is_null() {
        parent = gmem;
}

  if *size == -1 {

    
    *size = *gmem.size - *offset;
  }
  /* the shared memory is always readonly */
  
      GST_MEMORY_CAST (gst_apple_core_video_memory_new (GST_MINI_OBJECT_FLAGS
          (parent) | GST_MINI_OBJECT_FLAG_LOCK_READONLY, parent, mem->gpixbuf,
          mem->plane, gmem->maxsize, gmem->align, gmem->offset + offset, size));

  return sub;
}


unsafe extern "C" fn gst_apple_core_video_mem_is_span(mem1: *mut GstMemory, mem2: *mut GstMemory, size: *mut usize) -> glib::ffi::gboolean {
    false.into_glib()
}

unsafe extern "C" fn gst_apple_core_video_allocator_init (allocator: *mut GstAppleCoreVideoAllocator)
{
  let mut alloc: GstAllocator = ptr::read(allocator as *const _);

  alloc.mem_type = GST_APPLE_CORE_VIDEO_ALLOCATOR_NAME;
  alloc.mem_map = gst_apple_core_video_mem_map;
  alloc.mem_unmap = gst_apple_core_video_mem_unmap;
  alloc.mem_share = gst_apple_core_video_mem_share;
  alloc.mem_is_span = Some(gst_apple_core_video_mem_is_span);
  alloc.object.flags |= GST_ALLOCATOR_FLAG_CUSTOM_ALLOC;
}

