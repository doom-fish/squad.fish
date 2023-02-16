#![allow(dead_code)]
mod macros;
mod os_types;
mod sys;

use core_graphics_types::geometry::CGRect;
use objc_id::Id;

use sys::shareable_content::{
    UnsafeSCDisplay, UnsafeSCRunningApplication, UnsafeSCShareableContent, UnsafeSCWindow,
};

#[derive(Debug)]
pub struct SCRunningApplication {
    unsafe_ref: Id<UnsafeSCRunningApplication>,
    pub process_id: i32,
    pub bundle_identifier: Option<String>,
    pub application_name: Option<String>,
}

impl From<Id<UnsafeSCRunningApplication>> for SCRunningApplication {
    fn from(unsafe_ref: Id<UnsafeSCRunningApplication>) -> Self {
        SCRunningApplication {
            process_id: unsafe_ref.get_process_id(),
            bundle_identifier: unsafe_ref.get_bundle_identifier(),
            application_name: unsafe_ref.get_application_name(),
            unsafe_ref,
        }
    }
}

#[derive(Debug)]
pub struct SCWindow {
    unsafe_ref: Id<UnsafeSCWindow>,
    pub title: Option<String>,
    pub owning_application: Option<SCRunningApplication>,
    pub window_id: u32,
    pub window_layer: u32,
    pub is_active: bool,
    pub is_on_screen: bool,
}

impl From<Id<UnsafeSCWindow>> for SCWindow {
    fn from(unsafe_ref: Id<UnsafeSCWindow>) -> Self {
        SCWindow {
            title: unsafe_ref.get_title(),
            window_id: unsafe_ref.get_window_id(),
            window_layer: unsafe_ref.get_window_layer(),
            is_active: unsafe_ref.get_is_active() == 1,
            is_on_screen: unsafe_ref.get_is_on_screen() == 1,
            owning_application: unsafe_ref
                .get_owning_application()
                .map(SCRunningApplication::from),
            unsafe_ref,
        }
    }
}

#[derive(Debug)]
pub struct SCDisplay {
    unsafe_ref: Id<UnsafeSCDisplay>,
    pub display_id: u32,
    pub frame: CGRect,
    pub width: u32,
    pub height: u32,
}

impl From<Id<UnsafeSCDisplay>> for SCDisplay {
    fn from(unsafe_ref: Id<UnsafeSCDisplay>) -> Self {
        SCDisplay {
            display_id: unsafe_ref.get_display_id(),
            frame: unsafe_ref.get_frame(),
            width: unsafe_ref.get_width(),
            height: unsafe_ref.get_height(),
            unsafe_ref,
        }
    }
}

#[derive(Debug)]
pub struct SCShareableContent {
    unsafe_ref: Id<UnsafeSCShareableContent>,
    pub windows: Vec<SCWindow>,
    pub applications: Vec<SCRunningApplication>,
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
