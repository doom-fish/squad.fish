use gst::Object;
use gst::{glib, Allocator};

pub const APPLE_VIDEO_ALLOCATOR_NAME: &str = "AppleCoreVideoAllocator";

pub mod imp {
    use std::ffi::c_char;
    use std::sync::Once;

    use gst::ffi::{GstAllocator, GST_ALLOCATOR_FLAG_CUSTOM_ALLOC};
    use gst::glib::subclass::InitializingObject;
    use gst::glib::translate::ToGlibPtr;
    use gst::prelude::Cast;
    use gst::subclass::prelude::*;
    use gst::{glib, Allocator};

    use crate::screencast::memory::{self};

    use super::APPLE_VIDEO_ALLOCATOR_NAME;
    #[derive(Debug)]
    pub struct AppleVideoAllocator {
        _once_bound: Once,
    }

    impl ObjectImpl for AppleVideoAllocator {}

    #[glib::object_subclass]
    impl ObjectSubclass for AppleVideoAllocator {
        const NAME: &'static str = APPLE_VIDEO_ALLOCATOR_NAME;
        type Type = super::AppleVideoAllocator;
        type ParentType = Allocator;
        fn new() -> Self {
            Self {
                _once_bound: Once::new(),
            }
        }
        #[inline]
        fn instance_init(obj: &InitializingObject<Self>) {
            unsafe {
                let obj_ref = obj.as_ref();
                obj_ref.imp()._once_bound.call_once(|| {
                    let instance_ref: &Allocator = obj_ref.upcast_ref();
                    let raw_ptr: *mut GstAllocator = instance_ref.to_glib_none().0;
                    let root_ffi_allocator: &mut GstAllocator = &mut *raw_ptr;
                    root_ffi_allocator.object.flags |= GST_ALLOCATOR_FLAG_CUSTOM_ALLOC;
                    root_ffi_allocator.mem_type =
                        format!("{APPLE_VIDEO_ALLOCATOR_NAME}\0").as_ptr() as *const c_char;
                    root_ffi_allocator.mem_map = Some(memory::mem_map);
                    root_ffi_allocator.mem_unmap = Some(memory::mem_unmap);
                    root_ffi_allocator.mem_share = Some(memory::mem_share);
                    root_ffi_allocator.mem_is_span = Some(memory::mem_is_span);
                })
            };
        }
    }
    impl GstObjectImpl for AppleVideoAllocator {}
    impl AllocatorImpl for AppleVideoAllocator {
        fn free(&self, memory: gst::Memory) {
            self.parent_free(memory)
        }
    }
}

glib::wrapper! {
    pub struct AppleVideoAllocator(ObjectSubclass<imp::AppleVideoAllocator>) @extends Allocator, Object;
}

impl AppleVideoAllocator {
    pub fn register() {
        Allocator::register(APPLE_VIDEO_ALLOCATOR_NAME, Self::default());
    }
}
impl Default for AppleVideoAllocator {
    fn default() -> Self {
        glib::Object::new()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::fn_address_comparisons)]
    use std::ptr;

    use gst::glib::translate::ToGlibPtr;
    use gst::{
        ffi::{GstAllocator, GST_ALLOCATOR_FLAG_CUSTOM_ALLOC},
        prelude::Cast,
    };
    use screencapturekit::cm_sample_buffer::CMSampleBuffer;
    use screencapturekit::sc_types::rc::Id;

    use crate::screencast::memory::{self, AppleVideoMemory};

    use super::AppleVideoAllocator;
    use super::*;
    #[test]
    fn core_apple_media_allocator_register() {
        gst::init().expect("should work!");
        AppleVideoAllocator::register();
        assert!(Allocator::find(Some(APPLE_VIDEO_ALLOCATOR_NAME)).is_some());
    }
    #[test]
    fn core_video_memory_allocator_has_bound_map_funcs() {
        gst::init().expect("should work!");
        let allocator = AppleVideoAllocator::default();
        unsafe {
            let super_allocator: &Allocator = allocator.upcast_ref();
            let super_allocator_ptr: *mut GstAllocator = super_allocator.to_glib_none().0;
            let ffi_super_allocator = ptr::read(super_allocator_ptr);

            assert!(
                ffi_super_allocator.object.flags & GST_ALLOCATOR_FLAG_CUSTOM_ALLOC
                    == GST_ALLOCATOR_FLAG_CUSTOM_ALLOC
            );

            assert!(ffi_super_allocator.mem_map.unwrap() == memory::mem_map);
            assert!(ffi_super_allocator.mem_unmap.unwrap() == memory::mem_unmap);
            assert!(ffi_super_allocator.mem_is_span.unwrap() == memory::mem_is_span);
            assert!(ffi_super_allocator.mem_share.unwrap() == memory::mem_share);
        }
    }

    #[test]
    fn core_video_memory_test_memory_with_allocator() {
        gst::init().expect("should work!");
        AppleVideoAllocator::register();
        unsafe {
            let mem = AppleVideoMemory::new_wrapped(
                CMSampleBuffer::new(Id::from_ptr(ptr::null_mut())),
                1,
                100,
            );
            let _ = mem.map_readable();
        }
    }
}
