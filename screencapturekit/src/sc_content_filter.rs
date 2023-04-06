use screencapturekit_sys::{content_filter::{UnsafeContentFilter, UnsafeInitParams}, os_types::rc::Id};

use crate::{
    sc_display::SCDisplay, sc_running_application::SCRunningApplication, sc_window::SCWindow,
};

#[derive(Debug)]
pub struct SCContentFilter {
    pub(crate) _unsafe_ref: Id<UnsafeContentFilter>,
}

pub enum InitParams<'a> {
    DesktopIndependentWindow(&'a SCWindow),
    Display(&'a SCDisplay),
    DisplayIncludingWindows(&'a SCDisplay, &'a [SCWindow]),
    DisplayExcludingWindows(&'a SCDisplay, &'a [SCWindow]),
    DisplayIncludingApplicationsExceptingWindows(
        &'a SCDisplay,
        &'a [SCRunningApplication],
        &'a [SCWindow],
    ),
    DisplayExcludingApplicationsExceptingWindows(
        &'a SCDisplay,
        &'a [SCRunningApplication],
        &'a [SCWindow],
    ),
}
impl <'a> From<InitParams<'a>> for screencapturekit_sys::content_filter::UnsafeInitParams<'a> {
    fn from(value: InitParams) -> Self {
        match value {
            InitParams::DesktopIndependentWindow(x) => UnsafeInitParams::DesktopIndependentWindow(x._unsafe_ref.clone()),
            InitParams::Display(x) => UnsafeInitParams::Display(x._unsafe_ref.clone()),
            InitParams::DisplayIncludingWindows(_, _) => todo!(),
            InitParams::DisplayExcludingWindows(_, _) => todo!(),
            InitParams::DisplayIncludingApplicationsExceptingWindows(_, _, _) => todo!(),
            InitParams::DisplayExcludingApplicationsExceptingWindows(_, _, _) => todo!(),
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
        //SCContentFilter::new(Display(&display));
    }
}
