use std::{mem::size_of, ptr};

use gst::{
    ffi::{GstAllocator, GstAllocatorClass, GstMemory, GstMeta, GstMapFlags},
    glib::{
        ffi::{
            g_atomic_int_dec_and_test, g_atomic_int_inc, g_mutex_clear, g_slice_free1, gpointer,
            GMutex, gboolean,
        },
        translate::IntoGlib,
    },
};

use super::{
    core_video_buffer::gst_apple_core_video_pixel_buffer_unref,
    types::{CMBlockBufferRef, CMSampleBufferRef, CVImageBufferRef, CVPixelBufferRef},
};

pub(crate) const LOCK_STATE_NAMES: [&str; 3] =
    ["Unlocked", "Locked Read-Only", "Locked Read-Write"];

#[repr(C)]
enum GstAppleCoreVideoLockState {
    GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED,
    GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READONLY,
    GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READ_WRITE,
}

#[repr(C)]
pub struct GstAppleCoreVideoPixelBuffer {
    pub refcount: i32,
    pub mutex: *mut GMutex,
    pub lock_state: GstAppleCoreVideoLockState,
    pub buf: CVPixelBufferRef,
    pub lock_count: usize,
}

#[repr(C)]
pub struct GstAppleCoreVideoMemory {
    pub mem: GstMemory,
    pub gpixbuf: *mut GstAppleCoreVideoPixelBuffer,
    pub plane: usize,
}

fn gst_apple_core_video_pixel_buffer_new(buf: CVPixelBufferRef) -> GstAppleCoreVideoPixelBuffer {
    GstAppleCoreVideoPixelBuffer {
        refcount: 1,
        buf,
        mutex: ptr::null_mut(),
        lock_state: GstAppleCoreVideoLockState::GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED,
        lock_count: 0,
    }
}

#[repr(C)]
struct GstAppleCoreVideoAllocatorClass {
    parent_class: GstAllocatorClass,
}

#[repr(C)]
struct GstAppleCoreVideoAllocator {
    parent_instance: GstAllocator,
}

const GST_APPLE_CORE_VIDEO_ALLOCATOR_NAME: *const i8 =
    b"AppleCoreVideoMemory\0".as_ptr() as *const _;

extern "C" {
    fn CVPixelBufferRelease(pixel_buf: CVPixelBufferRef);
    fn CVPixelBufferGetPlaneCount(pixel_buf: CVPixelBufferRef) -> usize;
    fn CVPixelBufferGetBytesPerRowOfPlane(pixel_buf: CVPixelBufferRef, index: usize) -> usize;
    fn CVPixelBufferGetHeightOfPlane(pixel_buf: CVPixelBufferRef, index: usize) -> usize;
    fn CVPixelBufferIsPlanar(pixel_buf: CVPixelBufferRef) -> u8;
}

#[no_mangle]
unsafe extern "C" fn gst_apple_core_video_pixel_buffer_ref(
    gpixbuf: *mut GstAppleCoreVideoPixelBuffer,
) -> *mut GstAppleCoreVideoPixelBuffer {
        let mut buf: GstAppleCoreVideoPixelBuffer = ptr::read(gpixbuf as *const _);
        g_atomic_int_inc(&buf.refcount as *const _ as *mut _);
        return gpixbuf;
}

#[no_mangle]
unsafe fn gst_apple_core_video_pixel_buffer_unref(gpixbuf: *mut GstAppleCoreVideoPixelBuffer) {
    let mut buf: GstAppleCoreVideoPixelBuffer = ptr::read(gpixbuf as *const _);
    if glib::ffi::g_atomic_int_dec_and_test(&buf.refcount as *const _ as *mut _) == 1 {
        if matches!(
            buf.lock_state,
            GstAppleCoreVideoLockState::GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED
        ) {
            //PANIC
        }
        CVPixelBufferRelease(buf.buf);
        g_mutex_clear(buf.mutex);
        g_slice_free1(
            size_of::<GstAppleCoreVideoPixelBuffer>(),
            gpixbuf as gpointer,
        );
    }
}


unsafe extern "C" fn gst_apple_core_video_pixel_buffer_lock (gpixbuf: *mut GstAppleCoreVideoPixelBuffer,
    
    flags: GstMapFlags) -> gboolean
{
  let mut buf: GstAppleCoreVideoPixelBuffer = ptr::read(gpixbuf as *const _);
  g_mutex_lock (buf.mutex);

  match (buf.lock_state) {
    GstAppleCoreVideoLockState::GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED => {

      let lockFlags = if(flags & GST_MAP_WRITE == 1) {0} else{ kCVPixelBufferLock_ReadOnly};
      cvret = CVPixelBufferLockBaseAddress (gpixbuf->buf, lockFlags);
      if (cvret != kCVReturnSuccess) {
        g_mutex_unlock (&gpixbuf->mutex);
        /* TODO: Map kCVReturnError etc. into strings */
        GST_ERROR ("%p: unable to lock base address for pixbuf %p: %d", gpixbuf,
            gpixbuf->buf, cvret);
        return FALSE;
      }
      gpixbuf->lock_state =
          (flags & GST_MAP_WRITE) ?
          GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READ_WRITE :
          GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READONLY;
        },
    GstAppleCoreVideoLockState::GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READONLY => todo!(),
    GstAppleCoreVideoLockState::GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READ_WRITE => todo!(),
    // case GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED:
    //   lockFlags = (flags & GST_MAP_WRITE) ? 0 : kCVPixelBufferLock_ReadOnly;
    //   cvret = CVPixelBufferLockBaseAddress (gpixbuf->buf, lockFlags);
    //   if (cvret != kCVReturnSuccess) {
    //     g_mutex_unlock (&gpixbuf->mutex);
    //     /* TODO: Map kCVReturnError etc. into strings */
    //     GST_ERROR ("%p: unable to lock base address for pixbuf %p: %d", gpixbuf,
    //         gpixbuf->buf, cvret);
    //     return FALSE;
    //   }
    //   gpixbuf->lock_state =
    //       (flags & GST_MAP_WRITE) ?
    //       GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READ_WRITE :
    //       GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READONLY;
    //   break;
    //
    // case GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READONLY:
    //   if (flags & GST_MAP_WRITE) {
    //     g_mutex_unlock (&gpixbuf->mutex);
    //     GST_ERROR ("%p: pixel buffer %p already locked for read-only access",
    //         gpixbuf, gpixbuf->buf);
    //     return FALSE;
    //   }
    //   break;
    //
    // case GST_APPLE_CORE_VIDEO_MEMORY_LOCKED_READ_WRITE:
    //   break;                    /* nothing to do, already most permissive mapping */
  }

  g_atomic_int_inc (&gpixbuf->lock_count);

  g_mutex_unlock (&gpixbuf->mutex);

  GST_DEBUG ("%p: pixbuf %p, %s (%d times)",
      gpixbuf,
      gpixbuf->buf,
      _lock_state_names[gpixbuf->lock_state], gpixbuf->lock_count);

  return TRUE;
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

unsafe extern "C" fn gst_apple_core_video_mem_share(
    gmem: *mut GstMemory,
    offset: isize,
    size: isize,
) -> *mut GstMemory {
    todo!()
    //  let mem: GstAppleCoreVideoMemory = ptr::read(gmem as *const _);
    //  let basemem = ptr::read(gmem);
    //   /* find the real parent */
    //   let mut parent = basemem.parent;
    //   if parent.is_null() {
    //         parent = gmem;
    // }
    //
    //   if *size == -1 {
    //
    //
    //     *size = *gmem.size - *offset;
    //   }
    //   /* the shared memory is always readonly */
    //   let sub =
    //       GST_MEMORY_CAST (gst_apple_core_video_memory_new (GST_MINI_OBJECT_FLAGS
    //           (parent) | GST_MINI_OBJECT_FLAG_LOCK_READONLY, parent, mem->gpixbuf,
    //           mem->plane, gmem->maxsize, gmem->align, gmem->offset + offset, size));
    //
    //   return sub;
}

unsafe extern "C" fn gst_apple_core_video_mem_is_span(
    mem1: *mut GstMemory,
    mem2: *mut GstMemory,
    size: *mut usize,
) -> gboolean {
    false.into_glib()
}

unsafe extern "C" fn gst_apple_core_video_allocator_init(
    allocator: *mut GstAppleCoreVideoAllocator,
) {
    let mut alloc: GstAllocator = ptr::read(allocator as *const _);

    alloc.mem_type = GST_APPLE_CORE_VIDEO_ALLOCATOR_NAME;
    alloc.mem_map = gst_apple_core_video_mem_map;
    alloc.mem_unmap = gst_apple_core_video_mem_unmap;
    alloc.mem_share = Some(gst_apple_core_video_mem_share);
    alloc.mem_is_span = Some(gst_apple_core_video_mem_is_span);
    alloc.object.flags |= GST_ALLOCATOR_FLAG_CUSTOM_ALLOC;
}
