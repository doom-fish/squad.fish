use screencapturekit_sys::shareable_content::UnsafeSCRunningApplication;


#[derive(Debug)]
pub struct SCRunningApplication {
    unsafe_ref: ShareId<UnsafeSCRunningApplication>,
    pub process_id: i32,
    pub bundle_identifier: Option<String>,
    pub application_name: Option<String>,
}

impl From<ShareId<UnsafeSCRunningApplication>> for SCRunningApplication {
    fn from(unsafe_ref: ShareId<UnsafeSCRunningApplication>) -> Self {
        SCRunningApplication {
            process_id: unsafe_ref.get_process_id(),
            bundle_identifier: unsafe_ref.get_bundle_identifier(),
            application_name: unsafe_ref.get_application_name(),
            unsafe_ref,
        }
    }
}

