// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use gst::{glib, CapsFeatures};
use gst::subclass::prelude::*;
use gst_base::subclass::base_src::CreateSuccess;
use gst_base::subclass::prelude::*;
use gst_gl::ffi::{GST_CAPS_FEATURE_MEMORY_GL_MEMORY, GST_GL_TEXTURE_TARGET_RECTANGLE_STR};
use gst_gl::*;
use gst_video::VideoFormat;
use once_cell::sync::Lazy;

// This module contains the private implementation details of our element

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "gstscreencapturekitsrc",
        gst::DebugColorFlags::empty(),
        Some("GStreamer Screencapture Kit"),
    )
});

#[derive(Default)]
pub struct ScreenCaptureSrc {}

#[glib::object_subclass]
impl ObjectSubclass for ScreenCaptureSrc {
    const NAME: &'static str = "GstScreenCaptureKitSrc";
    type Type = super::ScreenCaptureSrc;
    type ParentType = gst_base::PushSrc;
}

impl GstObjectImpl for ScreenCaptureSrc {}

impl ObjectImpl for ScreenCaptureSrc {}

// Implementation of gst::Element virtual methods
impl ElementImpl for ScreenCaptureSrc {
    // Set the element specific metadata. This information is what
    // is visible from gst-inspect-1.0 and can also be programmatically
    // retrieved from the gst::Registry after initial registration
    // without having to load the plugin in memory.
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "ScreenCaptureKit Source",
                "Source/ScreenCapture",
                "Generates media from ScreenCaptureKit",
                "Per Johansson <per@doom.fish>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    // Create and add pad templates for our sink and source pad. These
    // are later used for actually creating the pads and beforehand
    // already provide information to GStreamer about all possible
    // pads that could exist for this type.
    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            let gl_caps = gst_video::VideoCapsBuilder::new()
                .features(["memory:GLMemory"])
                .format(VideoFormat::Uyvy)
                .field("texture-target", "rectangle")
                .build();


            let raw_caps = gst_video::video_make_raw_caps(&[
                VideoFormat::Nv12,
                VideoFormat::Uyvy,
                VideoFormat::Yuy2,
            ]).build();
            let mut full_caps = gst::Caps::new_empty();
            full_caps.merge(gl_caps);
            full_caps.merge(raw_caps);
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &full_caps,
            )
            .unwrap();

            vec![src_pad_template]
        });
        PAD_TEMPLATES.as_ref()
    }

    // Called whenever the state of the element should be changed. This allows for
    // starting up the element, allocating/deallocating resources or shutting down
    // the element again.
    fn change_state(
        &self,
        transition: gst::StateChange,
    ) -> Result<gst::StateChangeSuccess, gst::StateChangeError> {
        // Call the parent class' implementation of ::change_state()
        self.parent_change_state(transition)
    }
}

// Implementation of gst_base::BaseSrc virtual methods
impl BaseSrcImpl for ScreenCaptureSrc {
    fn set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        gst::debug!(CAT, imp: self, "Configuring for caps {}", caps);

        Ok(())
    }

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        gst::info!(CAT, imp: self, "Started");

        Ok(())
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        gst::info!(CAT, imp: self, "Stopped");

        Ok(())
    }

    fn query(&self, query: &mut gst::QueryRef) -> bool {
        BaseSrcImplExt::parent_query(self, query)
    }

    fn fixate(&self, caps: gst::Caps) -> gst::Caps {
        self.parent_fixate(caps)
    }

    fn is_seekable(&self) -> bool {
        false
    }

    fn unlock(&self) -> Result<(), gst::ErrorMessage> {
        Ok(())
    }

    fn unlock_stop(&self) -> Result<(), gst::ErrorMessage> {
        // This signals that unlocking is done, so we can reset
        // all values again.
        Ok(())
    }
}

impl PushSrcImpl for ScreenCaptureSrc {
    fn create(&self, buffer: Option<&mut gst::BufferRef>) -> Result<CreateSuccess, gst::FlowError> {
        Ok(CreateSuccess::FilledBuffer)
    }
}
