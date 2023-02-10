use std::error::Error;

use objc::runtime::Object;
use objc_id::Id;

use crate::shared::*;

use crate::shared::{DisplayID, Rect, WindowID};
use crate::sys::{UnsafeSCShareableContent, UnsafeSCWindow};

pub struct SCRunningApplication {
    process_id: isize,
    bundle_identifier: String,
    application_name: String,
}

pub struct SCWindow {
    unsafe_ptr: Id<UnsafeSCShareableContent>,
    pub title: Option<String>,
    pub owning_application: Option<SCRunningApplication>,
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

pub struct SCShareableContent {
    pub windows: Vec<SCWindow>,
    pub applications: Vec<SCRunningApplication>,
    pub displays: Vec<SCDisplay>,
}

impl SCShareableContent {
    pub fn current() -> SCShareableContent {
        let usc = UnsafeSCShareableContent::get().unwrap();
        let windows = usc
                .windows()
                .iter()
                .map(|sw:Id<UnsafeSCWindow>| SCWindow {
                    unsafe_ptr: sw,
                    title: todo!(),
                    owning_application: todo!(),
                    id: todo!(),
                    window_layer: todo!(),
                    is_active: todo!(),
                    is_on_screen: todo!(),
                })
                .collect();
        SCShareableContent {
            windows,
            applications: todo!(),
            displays: todo!(),
        }
    }
}
