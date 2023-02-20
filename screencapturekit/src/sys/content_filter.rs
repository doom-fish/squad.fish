use objc::{runtime::Class, *};
use objc_foundation::{INSObject, NSArray, INSArray};
use objc_id::{Id, Shared};

use super::shareable_content::{
    UnsafeSCDisplay, UnsafeSCRunningApplication, UnsafeSCShareableContent, UnsafeSCWindow,
};

#[derive(Debug)]
pub struct UnsafeContentFilter;
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
    Display(Id<UnsafeSCDisplay>),
    DisplayIncludingWindows(Id<UnsafeSCDisplay>, &'a [Id<UnsafeSCWindow, Shared>]),
    DisplayExcludingWindows(Id<UnsafeSCDisplay>, &'a [Id<UnsafeSCWindow, Shared>]),
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
                InitParams::DesktopIndependentWindow(window) => {
                    msg_send![self, initWithDesktopIndependentWindow: window]
                }
                InitParams::DisplayIncludingWindows(display, windows) => {
                    msg_send![self, initWithDisplay : display excludingWindows: NSArray::from_slice(windows)]
                },
                InitParams::DisplayExcludingWindows(_, _) => todo!(),
                InitParams::DisplayIncludingApplicationsExceptingWindows(_, _, _) => todo!(),
                InitParams::DisplayExcludingApplicationsExceptingWindows(_, _, _) => todo!(),
                InitParams::Display(_) => todo!(),
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
            .unwrap();
        let window = UnsafeSCShareableContent::get()
            .unwrap()
            .windows()
            .pop()
            .unwrap().share();
        a.init(InitParams::DisplayExcludingWindows(display, &[window]));
    }
}
