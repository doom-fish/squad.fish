use gst::ffi::GstMemory;
use gst::glib;
use gst::glib::translate::*;
use gst::{memory_object_wrapper, Memory, MemoryRef};

/**
 * GstAppleCoreVideoLockState:
 *
 * Specifies whether the backing CVPixelBuffer is locked for read-only
 * or read-write.
 *
 * Locking for reading only improves performance by preventing
 * Core Video from invalidating existing caches of the buffer’s contents.
 */
// pub enum GstAppleCoreVideoLockState {
//     Unlocked,
//     LockedReadonly,
//     LockedReadWrite,
// }
// type CVPixelBufferRef = *mut Object;

/**
 * GstAppleCoreVideoPixelBuffer:
 *
 * This structure wraps CVPixelBuffer, managing its lock states and reference count.
 * It will be referenced by one or more #GstAppleCoreVideoMemory.
 */
#[derive(Debug)]
struct GstAppleCoreVideoPixelBuffer {
    //buf: CVPixelBufferRef,
}
/**
 * GstAppleCoreVideoMemory:
 *
 * Represents a video plane or an entire (non-planar) video image,
 * backed by a CVPixelBuffer.
 *
 * This structure shares a #GstAppleCoreVideoPixelBuffer instance
 * with other instances.
 */
#[derive(Debug)]
#[repr(C)]
pub struct GstAppleCoreVideoMemory {
    mem: GstMemory,
    //   gpixbuf: Arc<GstAppleCoreVideoPixelBuffer>,
    // plane: isize,
}

macro_rules! skip_assert_initialized {
    () => {};
}
memory_object_wrapper!(
    AppleCoreVideoMemory,
    AppleCoreVideoMemoryRef,
    GstAppleCoreVideoMemory,
    |_| true,
    Memory,
    MemoryRef
);


