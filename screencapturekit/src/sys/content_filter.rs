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
    DisplayIncludingWindows(
        ShareId<UnsafeSCDisplay>,
        &'a [ShareId<UnsafeSCWindow>],
    ),
    DisplayExcludingWindows(
        ShareId<UnsafeSCDisplay>,
        &'a [ShareId<UnsafeSCWindow>],
    ),
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
                    let _: () = msg_send![self, initWithDisplay : display excludingApplications : NSArray::from_slice(applications) exceptingWindows:  NSArray::from_slice(windows)];
                }
                InitParams::DisplayExcludingApplicationsExceptingWindows(
                    display,
                    applications,
                    windows,
                ) => {
                    let _: () = msg_send![self, initWithDisplay : display includingApplications : NSArray::from_slice(applications) exceptingWindows: NSArray::from_slice(windows)];
                }
            }
        };
    }
}
#[cfg(test)]
mod test {

    use crate::sys::shareable_content::UnsafeSCShareableContent;

    use super::*;

    #[test]
    fn test_init() {
        let filter = UnsafeContentFilter::new();

        let sc = UnsafeSCShareableContent::get().expect("should get shareable content");
        let display = sc.displays().pop().unwrap().share();
        let windows: Vec<ShareId<UnsafeSCWindow>> =
            sc.windows().into_iter().take(2).map(|w| w.share()).collect();

        // let applications = sc
        //     .applications()
        //     .iter()
        //     .take(2)
        //     .map(|a| a.share())
        //     .collect();

        filter.init(InitParams::Display(display.clone()));
        filter.init(InitParams::DisplayIncludingWindows(display.clone(), &windows[..]));
        filter.init(InitParams::DisplayExcludingWindows(display.clone(), &windows[..]));
        // filter.init(InitParams::DesktopIndependentWindow(windows[0]));
        // filter.init(InitParams::DisplayIncludingApplicationsExceptingWindows(
        //     display,
        //     applications,
        //     windows,
        // ));
        // filter.init(InitParams::DisplayIncludingApplicationsExceptingWindows(
        //     display,
        //     applications,
        //     windows,
        // ));
    }
}
