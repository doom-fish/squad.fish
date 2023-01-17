/*
Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.

SPDX-License-Identifier: MIT OR Apache-2.0
*/

use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst::QueryRef;

use once_cell::sync::Lazy;

// This module contains the private implementation details of our element
static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "screen",
        gst::DebugColorFlags::empty(),
        Some("Screencast Element"),
    )
});

// Struct containing all the element data
pub struct Screencast {
    srcpad: gst::Pad,
}

impl Screencast {
    fn src_event(self: &Self, _pad: &gst::Pad, _event: gst::Event) -> bool {
        true
    }
    fn src_query(self: &Self, _pad: &gst::Pad, _query: &mut QueryRef) -> bool {
        true
    }
}

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
#[glib::object_subclass]
impl ObjectSubclass for Screencast {
    const NAME: &'static str = "GstScreen";
    type Type = super::Screencast;
    type ParentType = gst::Element;

    // Called when a new instance is to be created. We need to return an instance
    // of our struct here and also get the class struct passed in case it's needed
    fn with_class(klass: &Self::Class) -> Self {
        let templ = klass.pad_template("src").unwrap();
        let srcpad = gst::Pad::builder_with_template(&templ, Some("src"))
            .event_function(|pad, parent, event| {
                Screencast::catch_panic_pad_function(
                    parent,
                    || false,
                    |screencast| screencast.src_event(pad, event),
                )
            })
            .query_function(|pad, parent, query| {
                Screencast::catch_panic_pad_function(
                    parent,
                    || false,
                    |screencast| screencast.src_query(pad, query),
                )
            })
            .build();
            srcpad.push(buffer)
        // Return an instance of our struct and also include our debug category here.
        // The debug category will be used later whenever we need to put something
        // into the debug logs
        Self { srcpad }
    }
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for Screencast {
    // Called right after construction of a new instance
    fn constructed(&self) {
        // Call the parent class' ::constructed() implementation first
        self.parent_constructed();

        // Here we actually add the pads we created in Screencast::new() to the
        // element so that GStreamer is aware of their existence.
        let obj = self.obj();

        obj.add_pad(&self.srcpad).unwrap();
    }
}

impl GstObjectImpl for Screencast {}

// Implementation of gst::Element virtual methods
impl ElementImpl for Screencast {
    // Set the element specific metadata. This information is what
    // is visible from gst-inspect-1.0 and can also be programmatically
    // retrieved from the gst::Registry after initial registration
    // without having to load the plugin in memory.
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Screencast",
                "Generic",
                "Does nothing with the data",
                "Per Johansson <mail@perjohansson.net>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    // Create and add pad templates for our sink and source pad. These
    // are later used for actually creating the pads and beforehand
    // already provide information to GStreamer about all possible
    // pads that could exist for this type.
    //
    // Actual instances can create pads based on those pad templates
    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
            // Our element can accept any possible caps on both pads
            let caps = gst::Caps::new_any();
            let src_pad_template = gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &caps,
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
        gst::trace!(CAT, imp: self, "Changing state {:?}", transition);

        // Call the parent class' implementation of ::change_state()
        self.parent_change_state(transition)
    }
}
