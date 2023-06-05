// SPDX-License-Identi// SPDX-License-Identifier: MPL-2.0
#![allow(unused)]
use gst::prelude::*;
use objc::runtime::Object;
use std::fmt;
use std::mem;

use super::types::CVBufferRef;

#[repr(transparent)]
pub struct GstCoreVideoMeta(imp::GstCoreVideoMeta);

unsafe impl Send for GstCoreVideoMeta {}
unsafe impl Sync for GstCoreVideoMeta {}

impl GstCoreVideoMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        cvbuf: CVBufferRef,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        unsafe {
            // Manually dropping because gst_buffer_add_meta() takes ownership of the
            // content of the struct
            let mut params = mem::ManuallyDrop::new(imp::GstCoreVideoMetaParams {
                cvbuf,
                pixbuf: cvbuf,
            });

            let meta = gst::ffi::gst_buffer_add_meta(
                buffer.as_mut_ptr(),
                imp::core_video_meta_get_info(),
                &mut *params as *mut imp::GstCoreVideoMetaParams as glib::ffi::gpointer,
            ) as *mut imp::GstCoreVideoMeta;

            Self::from_mut_ptr(buffer, meta)
        }
    }
}

unsafe impl MetaAPI for GstCoreVideoMeta {
    type GstType = imp::GstCoreVideoMeta;

    fn meta_api() -> gst::glib::Type {
        imp::core_video_meta_api_get_type()
    }
}

impl fmt::Debug for GstCoreVideoMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GstCoreVideoMeta").finish()
    }
}

mod imp {
    use gst::ffi::gst_buffer_add_meta;
    use gst::ffi::GstMetaTransformCopy;
    use gst::glib::translate::*;
    use objc::runtime::Object;
    use once_cell::sync::Lazy;
    use std::ffi::c_void;
    use std::mem;
    use std::ptr;

    use crate::screencast::types::CVBufferRef;
    use crate::screencast::types::CVPixelBufferRef;

    pub struct GstCoreVideoMeta {
        parent: gst::ffi::GstMeta,
        pub cvbuf: CVBufferRef,
        pub pixbuf: CVPixelBufferRef,
    }

    #[repr(C)]
    pub struct GstCoreVideoMetaParams {
        pub cvbuf: CVBufferRef,
        pub pixbuf: CVPixelBufferRef,
    }

    pub(super) fn core_video_meta_api_get_type() -> gst::glib::Type {
        static TYPE: Lazy<gst::glib::Type> = Lazy::new(|| unsafe {
            let t = from_glib(gst::ffi::gst_meta_api_type_register(
                b"GstCoreVideoMetaAPI\0".as_ptr() as *const _,
                [b"memory".as_ptr(), ptr::null()].as_ptr() as *mut *const _,
            ));

            assert_ne!(t, gst::glib::Type::INVALID);

            t
        });

        *TYPE
    }

    extern "C" {
        fn CVBufferRelease(bufRef: CVBufferRef) -> c_void;
    }

    unsafe extern "C" fn core_video_meta_init(
        meta: *mut gst::ffi::GstMeta,
        params: glib::ffi::gpointer,
        _buffer: *mut gst::ffi::GstBuffer,
    ) -> glib::ffi::gboolean {
        assert!(!params.is_null());

        let meta = &mut *(meta as *mut GstCoreVideoMeta);
        let params = ptr::read(params as *const GstCoreVideoMetaParams);
        meta.cvbuf = params.cvbuf;
        meta.pixbuf = params.pixbuf;
        true.into_glib()
    }

    unsafe extern "C" fn core_video_meta_free(
        meta: *mut gst::ffi::GstMeta,
        _buffer: *mut gst::ffi::GstBuffer,
    ) {
        let meta = &mut *(meta as *mut GstCoreVideoMeta);
        CVBufferRelease(meta.cvbuf);
    }

    unsafe extern "C" fn core_video_meta_transform(
        transbuffer: *mut gst::ffi::GstBuffer,
        meta_ptr: *mut gst::ffi::GstMeta,
        buffer: *mut gst::ffi::GstBuffer,
        type_: glib::ffi::GQuark,
        data_ptr: glib::ffi::gpointer,
    ) -> glib::ffi::gboolean {
        let data: GstMetaTransformCopy = unsafe { ptr::read(data_ptr as *const _) };
        if (data.region == 0) {
            let meta: GstCoreVideoMeta = unsafe { ptr::read(meta_ptr as *const _) };

            // Manually dropping because gst_buffer_add_meta() takes ownership of the
            // content of the struct
            let mut params = mem::ManuallyDrop::new(GstCoreVideoMetaParams {
                cvbuf: meta.cvbuf,
                pixbuf: meta.pixbuf,
            });

            gst::ffi::gst_buffer_add_meta(
                transbuffer,
                core_video_meta_get_info(),
                &mut *params as *mut GstCoreVideoMetaParams as glib::ffi::gpointer,
            );
        } else {
            // WARN
        }
        true.into_glib()
    }
    ///§
    pub(super) fn core_video_meta_get_info() -> *const gst::ffi::GstMetaInfo {
        struct MetaInfo(ptr::NonNull<gst::ffi::GstMetaInfo>);
        unsafe impl Send for MetaInfo {}
        unsafe impl Sync for MetaInfo {}

        static META_INFO: Lazy<MetaInfo> = Lazy::new(|| unsafe {
            MetaInfo(
                ptr::NonNull::new(gst::ffi::gst_meta_register(
                    core_video_meta_api_get_type().into_glib(),
                    b"GstCoreVideoMeta\0".as_ptr() as *const _,
                    mem::size_of::<GstCoreVideoMeta>(),
                    Some(core_video_meta_init),
                    Some(core_video_meta_free),
                    Some(core_video_meta_transform),
                ) as *mut gst::ffi::GstMetaInfo)
                .expect("Failed to register meta API"),
            )
        });

        META_INFO.0.as_ptr()
    }
}
