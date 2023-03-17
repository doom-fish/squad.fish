
use sys::shareable_content::{
    UnsafeSCDisplay, UnsafeSCRunningApplication, UnsafeSCShareableContent, UnsafeSCWindow,
};

#[derive(Debug)]
pub struct SCShareableContent {
    unsafe_ref: Id<UnsafeSCShareableContent>,
    #[readonly.make]
    pub windows: Vec<SCWindow>,
    #[readonly.make]
    pub applications: Vec<SCRunningApplication>,
    #[readonly.make]
    pub displays: Vec<SCDisplay>,
}

impl SCShareableContent {
    pub fn current() -> Self {
        let unsafe_ref = UnsafeSCShareableContent::get().unwrap();
        let windows: Vec<SCWindow> = unsafe_ref
            .windows()
            .into_iter()
            .map(SCWindow::from)
            .collect();
        let applications = unsafe_ref
            .applications()
            .into_iter()
            .map(SCRunningApplication::from)
            .collect();
        let displays = unsafe_ref
            .displays()
            .into_iter()
            .map(SCDisplay::from)
            .collect();
        SCShareableContent {
            windows,
            applications,
            displays,
            unsafe_ref,
        }
    }
}
