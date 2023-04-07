use screencapturekit_sys::{
    content_filter::{UnsafeContentFilter, UnsafeInitParams},
    os_types::rc::{Id, ShareId},
    shareable_content::UnsafeSCWindow,
};

use crate::{
    sc_display::SCDisplay, sc_running_application::SCRunningApplication, sc_window::SCWindow,
};

#[derive(Debug)]
pub struct SCContentFilter {
    pub(crate) _unsafe_ref: Id<UnsafeContentFilter>,
}

pub enum InitParams {
    DesktopIndependentWindow(SCWindow),
    Display(SCDisplay),
    DisplayIncludingWindows(SCDisplay, Vec<SCWindow>),
    DisplayExcludingWindows(SCDisplay, Vec<SCWindow>),
    DisplayIncludingApplicationsExceptingWindows(
        SCDisplay,
        Vec<SCRunningApplication>,
        Vec<SCWindow>,
    ),
    DisplayExcludingApplicationsExceptingWindows(
        SCDisplay,
        Vec<SCRunningApplication>,
        Vec<SCWindow>,
    ),
}
impl From<InitParams> for screencapturekit_sys::content_filter::UnsafeInitParams {
    fn from(value: InitParams) -> Self {
        match value {
            InitParams::DesktopIndependentWindow(w) => {
                UnsafeInitParams::DesktopIndependentWindow(w._unsafe_ref)
            }
            InitParams::Display(d) => UnsafeInitParams::Display(d._unsafe_ref),
            InitParams::DisplayIncludingWindows(d, w) => UnsafeInitParams::DisplayIncludingWindows(
                d._unsafe_ref,
                w.into_iter().map(|w| w._unsafe_ref).collect(),
            ),
            InitParams::DisplayExcludingWindows(d, w) => UnsafeInitParams::DisplayExcludingWindows(
                d._unsafe_ref,
                w.into_iter().map(|w| w._unsafe_ref).collect(),
            ),
            InitParams::DisplayIncludingApplicationsExceptingWindows(d, a, w) => UnsafeInitParams::DisplayIncludingApplicationsExceptingWindows((), (), ()),
            InitParams::DisplayExcludingApplicationsExceptingWindows(d, a, w) => todo!(),
        }
    }
}
impl SCContentFilter {
    pub fn new(params: InitParams) -> Self {
        Self {
            _unsafe_ref: UnsafeContentFilter::init(params.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sc_shareable_content::SCShareableContent;

    use super::InitParams::Display;
    use super::*;
    #[test]
    fn test_sc_filter() {
        let display = SCShareableContent::current().displays.pop().unwrap();
        SCContentFilter::new(Display(&display));
    }
}
