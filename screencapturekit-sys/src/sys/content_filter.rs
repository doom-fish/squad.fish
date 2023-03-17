use objc::{runtime::Class, *};
use objc_foundation::{INSArray, INSObject, NSArray};
use objc_id::{Id, ShareId, Shared};

use super::shareable_content::{UnsafeSCDisplay, UnsafeSCRunningApplication, UnsafeSCWindow};

#[derive(Debug)]
pub struct UnsafeContentFilter {
    __priv: u8,
}
unsafe impl Message for UnsafeContentFilter {}
impl UnsafeContentFilter {}

impl INSObject for UnsafeContentFilter {
    fn class() -> &'static Class {
        Class::get("SCContentFilter").expect(
            "Missing SCContentFilter class, check that the binary is linked with ScreenCaptureKit",
        )
    }
}
pub enum InitParams<'a> {
    DesktopIndependentWindow(ShareId<UnsafeSCWindow>),
    Display(ShareId<UnsafeSCDisplay>),
    DisplayIncludingWindows(ShareId<UnsafeSCDisplay>, &'a [ShareId<UnsafeSCWindow>]),
    DisplayExcludingWindows(ShareId<UnsafeSCDisplay>, &'a [ShareId<UnsafeSCWindow>]),
    DisplayIncludingApplicationsExceptingWindows(
        ShareId<UnsafeSCDisplay>,
        &'a [ShareId<UnsafeSCRunningApplication>],
        &'a [ShareId<UnsafeSCWindow>],
    ),
    DisplayExcludingApplicationsExceptingWindows(
        ShareId<UnsafeSCDisplay>,
        &'a [ShareId<UnsafeSCRunningApplication>],
        &'a [ShareId<UnsafeSCWindow>],
    ),
}

impl UnsafeContentFilter {
    pub fn init(params: InitParams) -> Id<Self> {
        let content_filter = UnsafeContentFilter::new();
        unsafe {
            match params {
                InitParams::Display(display) => {
                    let _: () = msg_send![content_filter, initWithDisplay: display excludingWindows: NSArray::from_slice(&[] as &[Id<UnsafeSCWindow, Shared>])];
                }
                InitParams::DesktopIndependentWindow(window) => {
                    let _: () = msg_send![content_filter, initWithDesktopIndependentWindow: window];
                }
                InitParams::DisplayIncludingWindows(display, windows) => {
                    let _: () = msg_send![content_filter, initWithDisplay : display includingWindows: NSArray::from_slice(windows)];
                }
                InitParams::DisplayExcludingWindows(display, windows) => {
                    let _: () = msg_send![content_filter, initWithDisplay : display excludingWindows: NSArray::from_slice(windows)];
                }
                InitParams::DisplayIncludingApplicationsExceptingWindows(
                    display,
                    applications,
                    windows,
                ) => {
                    let _: () = msg_send![content_filter, initWithDisplay : display excludingApplications : NSArray::from_slice(applications) exceptingWindows:  NSArray::from_slice(windows)];
                }
                InitParams::DisplayExcludingApplicationsExceptingWindows(
                    display,
                    applications,
                    windows,
                ) => {
                    let _: () = msg_send![content_filter, initWithDisplay : display includingApplications : NSArray::from_slice(applications) exceptingWindows: NSArray::from_slice(windows)];
                }
            }
        };
        content_filter
    }
}
#[cfg(test)]
mod test {

    use crate::sys::shareable_content::UnsafeSCShareableContent;

    use super::*;

    #[test]
    fn test_init() {
        
        let sc = UnsafeSCShareableContent::get().expect("should get shareable content");
        let applications = sc.applications();
        let windows = sc.windows();
        let display = sc.displays().pop().unwrap();

        UnsafeContentFilter::init(InitParams::DisplayIncludingWindows(
            display.clone(),
            &windows[..],
        ));
        UnsafeContentFilter::init(InitParams::DisplayExcludingWindows(
            display.clone(),
            &windows[..2],
        ));
        UnsafeContentFilter::init(InitParams::DesktopIndependentWindow(windows[0].clone()));
        UnsafeContentFilter::init(InitParams::DisplayIncludingApplicationsExceptingWindows(
            display.clone(),
            &applications[..2],
            &windows[..2],
        ));
        UnsafeContentFilter::init(InitParams::DisplayIncludingApplicationsExceptingWindows(
            display.clone(),
            &applications[..2],
            &windows[..2],
        ));

        drop(sc);
        drop(applications);
        drop(windows);
        drop(display);
    }
}
