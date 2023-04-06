use screencapturekit_sys::{content_filter::UnsafeContentFilter, os_types::rc::Id};

#[derive(Debug)]
pub struct SCContentFilter {
    pub(crate) _unsafe_ref: Id<UnsafeContentFilter>,
}

impl SCContentFilter {
    pub fn new() -> Self {}
} 
