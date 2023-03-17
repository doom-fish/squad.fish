use screencapturekit_sys::content_filter::UnsafeContentFilter;

#[derive(Debug)]
pub struct SCContentFilter {
    pub(crate) _unsafe_ref: UnsafeContentFilter,
}
