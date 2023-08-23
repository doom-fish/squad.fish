use gst::Object;
use gst::{glib, Allocator};
pub mod imp {
    use std::sync::Once;

    use gst::ffi::{GstAllocator, GstMemory, GST_ALLOCATOR_FLAG_CUSTOM_ALLOC};
    use gst::glib::ffi::{gboolean, gpointer};
    use gst::glib::subclass::InitializingObject;
    use gst::glib::translate::{IntoGlib, ToGlibPtr};
    use gst::prelude::Cast;
    use gst::subclass::prelude::*;
    use gst::{glib, Allocator};

    #[derive(Debug)]
    pub struct AppleCoreVideoAllocator {
        _once_bound: Once,
    }

    impl AppleCoreVideoAllocator {
        pub(super) unsafe extern "C" fn mem_map(
            _mem: *mut GstMemory,
            _size: usize,
            _flags: u32,
        ) -> gpointer {
            // let memory = Memory::from_glib_none(mem);
            // let core_video_memory = memory.downcast_memory::<AppleCoreVideoMemory>().unwrap();

            // let raw: *mut GstAppleCoreVideoMemory = core_video_memory.to_glib_none().0;

            std::ptr::null_mut()
        }
        pub(super) unsafe extern "C" fn mem_unmap(_mem: *mut GstMemory) {
            todo!()
        }
        pub(super) unsafe extern "C" fn mem_share(
            _mem: *mut GstMemory,
            _offset: isize,
            _size: isize,
        ) -> *mut GstMemory {
            todo!()
        }
        pub(super) unsafe extern "C" fn mem_is_span(
            _mem1: *mut GstMemory,
            _mem2: *mut GstMemory,
            _offset: *mut usize,
        ) -> gboolean {
            false.into_glib()
        }
    }
    impl ObjectImpl for AppleCoreVideoAllocator {}
    #[glib::object_subclass]
    impl ObjectSubclass for AppleCoreVideoAllocator {
        const NAME: &'static str = "AppleCoreVideoAllocator";
        type Type = super::AppleCoreVideoAllocator;
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
                    root_ffi_allocator.mem_type = Self::NAME.as_ptr() as *const i8;
                    root_ffi_allocator.mem_map = Some(Self::mem_map);
                    root_ffi_allocator.mem_unmap = Some(Self::mem_unmap);
                    root_ffi_allocator.mem_share = Some(Self::mem_share);
                    root_ffi_allocator.mem_is_span = Some(Self::mem_is_span);
                })
            };
        }
    }
    impl GstObjectImpl for AppleCoreVideoAllocator {}
    impl AllocatorImpl for AppleCoreVideoAllocator {
        fn free(&self, memory: gst::Memory) {
            self.parent_free(memory)
        }
    }
}

glib::wrapper! {
    pub struct AppleCoreVideoAllocator(ObjectSubclass<imp::AppleCoreVideoAllocator>) @extends Allocator, Object;
}

impl Default for AppleCoreVideoAllocator {
    fn default() -> Self {
        glib::Object::new()
    }
}
#[cfg(test)]
mod tests {
    #![allow(clippy::fn_address_comparisons)]
    use std::ptr;

    use gst::{
        ffi::{GstAllocator, GST_ALLOCATOR_FLAG_CUSTOM_ALLOC},
        glib::translate::ToGlibPtr,
        prelude::Cast,
    };
    use gst_base::prelude::GstObjectExt;

    use super::*;
    #[test]
    fn core_apple_media_allocator_register() {
        gst::init().expect("should work!");

        let allocator = AppleCoreVideoAllocator::default();
        let allocator_name = &allocator.name();
        Allocator::register(allocator_name.as_str(), allocator);

        assert!(Allocator::find(Some(allocator_name.as_str())).is_some());
    }
    #[test]
    fn core_video_memory_allocator_has_bound_map_funcs() {
        let allocator = AppleCoreVideoAllocator::default();
        unsafe {
            let super_allocator: &Allocator = allocator.unsafe_cast_ref();
            let super_allocator_ptr: *mut GstAllocator = super_allocator.to_glib_none().0;
            let ffi_super_allocator = ptr::read(super_allocator_ptr);

            assert!(
                ffi_super_allocator.object.flags & GST_ALLOCATOR_FLAG_CUSTOM_ALLOC
                    == GST_ALLOCATOR_FLAG_CUSTOM_ALLOC
            );

            assert!(ffi_super_allocator.mem_map.unwrap() == imp::AppleCoreVideoAllocator::mem_map);
            assert!(
                ffi_super_allocator.mem_unmap.unwrap() == imp::AppleCoreVideoAllocator::mem_unmap
            );
            assert!(
                ffi_super_allocator.mem_is_span.unwrap()
                    == imp::AppleCoreVideoAllocator::mem_is_span
            );
            assert!(
                ffi_super_allocator.mem_share.unwrap() == imp::AppleCoreVideoAllocator::mem_share
            );
        }
    }
}
