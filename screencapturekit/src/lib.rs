#![allow(dead_code)]
use block::ConcreteBlock;
use objc::{
    runtime::{Class, Object},
    *,
};
use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSString};
use objc_id::Id;
use std::sync::mpsc::{channel, RecvError};

type WindowID = u32;
type DisplayID = u32;

struct NSError;

#[derive(Debug)]
pub struct Size {
    width: f64,
    height: f64,
}

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}
#[derive(Debug)]
pub struct Rect {
    origin: Point,
    size: Size,
}

pub struct SCRunningApplication<'a> {
    process_id: isize,
    bundle_identifier: &'a str,
    application_name: &'a str,
}

pub struct SCWindow<'a> {
    ptr: Id<Object>,
    pub title: Option<&'a str>,
    pub owning_application: Option<&'a SCRunningApplication<'a>>,
    pub id: WindowID,
    pub window_layer: u32,
    pub is_active: bool,
    pub is_on_screen: bool,
}
pub struct SCDisplay {
    pub display_id: DisplayID,
    pub frame: Rect,
    pub width: u64,
    pub height: u64,
}
struct RawSCRunningApplication;
unsafe impl Message for RawSCRunningApplication {}

macro_rules! get_string {
    // The `expr` designator is used for expressions.
    ($obj:ident, $name: ident) => {{
        let string_ptr: *const NSString = msg_send![$obj, $name];
        if string_ptr.is_null() {
            None
        } else {
            Some((*string_ptr).as_str())
        }
    }};
}

impl RawSCRunningApplication {
    fn get_process_id(&self) -> isize {
        unsafe { msg_send![self, processID] }
    }
    fn get_application_name(&self) -> Option<&str> {
        unsafe { get_string!(self, applicationName) }
    }
    fn get_bundle_identifier(&self) -> Option<&str> {
        unsafe { get_string!(self, bundleIdentifier) }
    }
}

impl INSObject for RawSCRunningApplication {
    fn class() -> &'static runtime::Class {
        Class::get("SCRunningApplication")
            .expect("Missing SCRunningApplication class, check that the binary is linked with ScreenCaptureKit")
    }
}
struct RawSCWindow;
unsafe impl Message for RawSCWindow {}

impl RawSCWindow {
    fn get_owning_application(&self) -> Id<RawSCRunningApplication> {
        unsafe { Id::from_ptr(msg_send![self, owningApplication]) }
    }
    fn get_window_layer(&self) -> u32 {
        unsafe { msg_send![self, windowLayer] }
    }
    fn get_window_id(&self) -> WindowID {
        unsafe { msg_send![self, windowID] }
    }
    fn get_title(&self) -> Option<&str> {
        unsafe { get_string!(self, title) }
    }
}

impl INSObject for RawSCWindow {
    fn class() -> &'static runtime::Class {
        Class::get("SCWindow")
            .expect("Missing SCWindow class, check that the binary is linked with ScreenCaptureKit")
    }
}

struct RawSCDisplay;
unsafe impl Message for RawSCDisplay {}

impl RawSCDisplay {
    fn get_display_id(&self) -> DisplayID {
        unsafe { msg_send![self, displayID] }
    }
    fn get_frame(&self) -> Rect {
        unsafe { msg_send![self, frame] }
    }
    fn get_height(&self) -> u32 {
        unsafe { msg_send![self, height] }
    }
    fn get_width(&self) -> u32 {
        unsafe { msg_send![self, width] }
    }
}

impl INSObject for RawSCDisplay {
    fn class() -> &'static runtime::Class {
        Class::get("SCDisplay")
            .expect("Missing SCWindow class, check that the binary is linked with ScreenCaptureKit")
    }
}

enum OnScreenOnlySettings<'a> {
    Every,
    Above(&'a RawSCWindow),
    Below(&'a RawSCWindow),
}

struct ExcludingDesktopWindowsConfig<'a> {
    exclude_desktop_windows: bool,
    on_screen_windows_only: Option<OnScreenOnlySettings<'a>>,
}

struct RawSCShareableContent;
unsafe impl Message for RawSCShareableContent {}
impl RawSCShareableContent {
    fn get(config: Option<ExcludingDesktopWindowsConfig>) -> Result<Id<Self>, RecvError> {
        let (tx, rx) = channel();

        unsafe {
            let handler = ConcreteBlock::new(move |sc: *mut RawSCShareableContent, _: NSError| {
                tx.send(Id::from_ptr(sc)).expect("Should work!");
            });
            let _: () = if let Some(c) = config {
                if let Some(only_screen) = c.on_screen_windows_only {
                    match only_screen {
                        OnScreenOnlySettings::Every => msg_send![
                            class!(SCShareableContent),
                            getShareableContentExcludingDesktopWindows: c.exclude_desktop_windows as u8
                            onScreenWindowsOnly: 1
                            completionHandler: &*handler.copy()
                        ],

                        OnScreenOnlySettings::Above(ref w) => msg_send![
                            class!(SCShareableContent),
                            getShareableContentExcludingDesktopWindows: c.exclude_desktop_windows as u8
                            onScreenWindowsOnlyAboveWindow: &w
                            completionHandler: &*handler.copy()
                        ],
                        OnScreenOnlySettings::Below(ref w) => msg_send![
                            class!(SCShareableContent),
                            getShareableContentExcludingDesktopWindows: c.exclude_desktop_windows as u8
                            onScreenWindowsOnlyBelowWindow: &w
                            completionHandler: &*handler.copy()
                        ],
                    }
                } else {
                    msg_send![
                        class!(SCShareableContent),
                        getShareableContentExcludingDesktopWindows: c.exclude_desktop_windows as u8
                        onScreenWindowsOnly: 0
                        completionHandler: &*handler.copy()
                    ]
                }
            } else {
                msg_send![
                    class!(SCShareableContent),
                    getShareableContentWithCompletionHandler: &*handler.copy()
                ]
            };
        };

        rx.recv()
    }

    fn displays(&self) -> Vec<Id<RawSCDisplay>> {
        let display_ptr: Id<NSArray<RawSCDisplay>> =
            unsafe { Id::from_ptr(msg_send!(self, displays)) };

        INSArray::into_vec(display_ptr)
    }
    fn applications(&self) -> Vec<Id<RawSCRunningApplication>> {
        let applications_ptr: Id<NSArray<RawSCRunningApplication>> =
            unsafe { Id::from_ptr(msg_send!(self, applications)) };

        INSArray::into_vec(applications_ptr)
    }
    fn windows(&self) -> Vec<Id<RawSCWindow>> {
        let windows_ptr: Id<NSArray<RawSCWindow>> =
            unsafe { Id::from_ptr(msg_send!(self, windows)) };

        INSArray::into_vec(windows_ptr)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_get_windows() {
        let sc = RawSCShareableContent::get(None).expect("Should be able to get sharable content");
        for w in sc.windows().iter() {
            assert!(
                w.get_title().is_some() || w.get_title().is_none(),
                "Can get title"
            );
        }
    }

    #[test]
    fn test_get_displays() {
        let sc = RawSCShareableContent::get(None).expect("Should be able to get sharable content");
        for d in sc.displays().iter() {
            println!("frame: {:?}", d.get_frame());
            assert!(d.get_frame().size.width > 0f64, "Can get application_name");
        }
    }
    #[test]
    fn test_get_applications() {
        let sc = RawSCShareableContent::get(None).expect("Should be able to get sharable content");
        for a in sc.applications().iter() {
            assert!(
                a.get_application_name().is_some() || a.get_application_name().is_none(),
                "Can get application_name"
            );
        }
    }
}
