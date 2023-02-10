use objc::runtime::Object;
use objc_id::Id;

use crate::shared::*;

use crate::shared::{DisplayID, Rect, WindowID};
use crate::sys::UnsafeSCShareableContent;

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

pub struct SCShareableContent<'a> {
    pub windows: &'a [SCWindow<'a>],
    pub applications: &'a [SCRunningApplication<'a>],
    pub displays: &'a [SCDisplay],
}

impl SCShareableContent<'_> {
} 
