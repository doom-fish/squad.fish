use std::sync::mpsc::{channel, Receiver, RecvError};

use block::{ConcreteBlock, RcBlock};
use objc::{
    msg_send,
    runtime::{Class, Object},
    Message, *,
};
use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSString};
use objc_id::*;

use crate::shared::{DisplayID, Rect, WindowID};

pub struct UnsafeSCRunningApplication;
unsafe impl Message for UnsafeSCRunningApplication {}

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

impl UnsafeSCRunningApplication {
    pub fn get_process_id(&self) -> isize {
        unsafe { msg_send![self, processID] }
    }
    pub fn get_application_name(&self) -> Option<&str> {
        unsafe { get_string!(self, applicationName) }
    }
    pub fn get_bundle_identifier(&self) -> Option<&str> {
        unsafe { get_string!(self, bundleIdentifier) }
    }
}

impl INSObject for UnsafeSCRunningApplication {
    fn class() -> &'static Class {
        Class::get("SCRunningApplication")
                .expect("Missing SCRunningApplication class, check that the binary is linked with ScreenCaptureKit")
    }
}
pub struct UnsafeSCWindow;
unsafe impl Message for UnsafeSCWindow {}

impl UnsafeSCWindow {
    fn get_owning_application(&self) -> Id<UnsafeSCRunningApplication> {
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

impl INSObject for UnsafeSCWindow {
    fn class() -> &'static runtime::Class {
        Class::get("SCWindow")
            .expect("Missing SCWindow class, check that the binary is linked with ScreenCaptureKit")
    }
}

pub struct UnsafeSCDisplay;
unsafe impl Message for UnsafeSCDisplay {}

impl UnsafeSCDisplay {
    pub fn get_display_id(&self) -> DisplayID {
        unsafe { msg_send![self, displayID] }
    }
    pub fn get_frame(&self) -> Rect {
        unsafe { msg_send![self, frame] }
    }
    pub fn get_height(&self) -> u32 {
        unsafe { msg_send![self, height] }
    }
    pub fn get_width(&self) -> u32 {
        unsafe { msg_send![self, width] }
    }
}

impl INSObject for UnsafeSCDisplay {
    fn class() -> &'static runtime::Class {
        Class::get("SCDisplay")
            .expect("Missing SCWindow class, check that the binary is linked with ScreenCaptureKit")
    }
}

#[derive(Default)]
pub enum OnScreenOnlySettings<'a> {
    EveryWindow,
    #[default]
    OnlyOnScreen,
    AboveWindow(&'a UnsafeSCWindow),
    BelowWindow(&'a UnsafeSCWindow),
}
#[derive(Default)]
pub struct ExcludingDesktopWindowsConfig<'a> {
    exclude_desktop_windows: bool,
    on_screen_windows_only: OnScreenOnlySettings<'a>,
}

pub struct UnsafeSCShareableContent;
unsafe impl Message for UnsafeSCShareableContent {}
type CompletionHandlerBlock = RcBlock<(*mut UnsafeSCShareableContent, *mut Object), ()>;
impl UnsafeSCShareableContent {
    unsafe fn new_completion_handler() -> (CompletionHandlerBlock, Receiver<Id<Self>>) {
        let (tx, rx) = channel();
        let handler = ConcreteBlock::new(move |sc: *mut Self, _error: *mut Object| {
            tx.send(Id::from_ptr(sc)).expect("Should work!");
        });
        (handler.copy(), rx)
    }

    pub fn get_with_config(config: &ExcludingDesktopWindowsConfig) -> Result<Id<Self>, RecvError> {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            match config.on_screen_windows_only {
                OnScreenOnlySettings::EveryWindow => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnly: 0
                    completionHandler: handler
                ],

                OnScreenOnlySettings::AboveWindow(ref w) => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnlyAboveWindow: &w
                    completionHandler: handler
                ],
                OnScreenOnlySettings::BelowWindow(ref w) => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnlyBelowWindow: &w
                    completionHandler: handler
                ],
                OnScreenOnlySettings::OnlyOnScreen => msg_send![
                    class!(SCShareableContent),
                    getShareableContentExcludingDesktopWindows: config.exclude_desktop_windows as u8
                    onScreenWindowsOnly: 1
                    completionHandler: handler
                ],
            }
            rx.recv()
         }
    }
    pub fn get() -> Result<Id<Self>, RecvError> {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            let _: () = msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: handler
            ];

            rx.recv()
        }
    }

    pub fn displays(&self) -> Vec<Id<UnsafeSCDisplay>> {
        let display_ptr: Id<NSArray<UnsafeSCDisplay>> =
            unsafe { Id::from_ptr(msg_send!(self, displays)) };

        INSArray::into_vec(display_ptr)
    }
    pub fn applications(&self) -> Vec<Id<UnsafeSCRunningApplication>> {
        let applications_ptr: Id<NSArray<UnsafeSCRunningApplication>> =
            unsafe { Id::from_ptr(msg_send!(self, applications)) };

        INSArray::into_vec(applications_ptr)
    }
    pub fn windows(&self) -> Vec<Id<UnsafeSCWindow>> {
        let windows_ptr: Id<NSArray<UnsafeSCWindow>> =
            unsafe { Id::from_ptr(msg_send!(self, windows)) };

        INSArray::into_vec(windows_ptr)
    }
}

#[cfg(test)]
mod get_shareable_content_with_config {
    use super::*;
    #[test]
    fn get_exclude_desktop_windows() {
        let mut config = ExcludingDesktopWindowsConfig::default();

        let _ = UnsafeSCShareableContent::get_with_config(&config);

        config.exclude_desktop_windows = true;
        let _ = UnsafeSCShareableContent::get_with_config(&config);

        config.exclude_desktop_windows = true;
        config.on_screen_windows_only = OnScreenOnlySettings::EveryWindow;
        let _ = UnsafeSCShareableContent::get_with_config(&config);
    }
}
#[cfg(test)]
mod get_shareable_content {

    use super::*;
    #[test]
    fn test_get_windows() {
        let sc = UnsafeSCShareableContent::get().expect("Should be able to get sharable content");
        for w in sc.windows().iter() {
            assert!(
                w.get_title().is_some() || w.get_title().is_none(),
                "Can get title"
            );
        }
    }

    #[test]
    fn test_get_displays() {
        let sc = UnsafeSCShareableContent::get().expect("Should be able to get sharable content");
        for d in sc.displays().iter() {
            println!("frame: {:?}", d.get_frame());
            assert!(d.get_frame().size.width > 0f64, "Can get application_name");
        }
    }
    #[test]
    fn test_get_applications() {
        let sc = UnsafeSCShareableContent::get().expect("Should be able to get sharable content");
        for a in sc.applications().iter() {
            assert!(
                a.get_application_name().is_some() || a.get_application_name().is_none(),
                "Can get application_name"
            );
        }
    }
}
