use objc::{runtime::Class, *};
use objc_foundation::{INSArray, INSObject, NSArray};
use objc_id::{Id, Shared};

use super::shareable_content::{
    UnsafeSCDisplay, UnsafeSCRunningApplication, UnsafeSCShareableContent, UnsafeSCWindow,
};

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
    DesktopIndependentWindow(Id<UnsafeSCWindow, Shared>),
    Display(Id<UnsafeSCDisplay, Shared>),
    DisplayIncludingWindows(
        Id<UnsafeSCDisplay, Shared>,
        &'a [Id<UnsafeSCWindow, Shared>],
    ),
    DisplayExcludingWindows(
        Id<UnsafeSCDisplay, Shared>,
        &'a [Id<UnsafeSCWindow, Shared>],
    ),
    DisplayIncludingApplicationsExceptingWindows(
        Id<UnsafeSCDisplay, Shared>,
        &'a [Id<UnsafeSCRunningApplication, Shared>],
        &'a [Id<UnsafeSCWindow, Shared>],
    ),
    DisplayExcludingApplicationsExceptingWindows(
        Id<UnsafeSCDisplay, Shared>,
        &'a [Id<UnsafeSCRunningApplication, Shared>],
        &'a [Id<UnsafeSCWindow, Shared>],
    ),
}
impl UnsafeContentFilter {
    fn init(&self, params: InitParams) {
        unsafe {
            match params {
                InitParams::Display(display) => {
                    let _: () = msg_send![self, initWithDisplay: display excludingWindows: NSArray::from_slice(&[] as &[Id<UnsafeSCWindow, Shared>])];
                }
                InitParams::DesktopIndependentWindow(window) => {
                    let _: () = msg_send![self, initWithDesktopIndependentWindow: window];
                }
                InitParams::DisplayIncludingWindows(display, windows) => {
                    let _: () = msg_send![self, initWithDisplay : display includingWindows: NSArray::from_slice(windows)];
                }
                InitParams::DisplayExcludingWindows(display, windows) => {
                    let _: () = msg_send![self, initWithDisplay : display excludingWindows: NSArray::from_slice(windows)];
                }
                InitParams::DisplayIncludingApplicationsExceptingWindows(
                    display,
                    applications,
                    windows,
                ) => {
                    let _: () = msg_send![self, dinitWithDisplay : display excludingApplications : NSArray::from_slice(applications) exceptingWindows:  NSArray::from_slice(windows)];
                }
                InitParams::DisplayExcludingApplicationsExceptingWindows(
                    display,
                    applications,
                    windows,
                ) => {
                    let _: () = msg_send![self, dinitWithDisplay : display includingApplications : NSArray::from_slice(applications) exceptingWindows: NSArray::from_slice(windows)];
                }
            }
        };
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_init() {
        let a = UnsafeContentFilter::new();

        let display = UnsafeSCShareableContent::get()
            .unwrap()
            .displays()
            .pop()
            .unwrap()
            .share();
        a.init(InitParams::Display(display));
    }
}
