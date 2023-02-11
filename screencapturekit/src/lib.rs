mod shared;
mod sys;

use std::error::Error;

use objc::runtime::Object;
use objc_id::{Id, Owned, Shared};
use sys::UnsafeSCRunningApplication;

use crate::shared::*;

use crate::shared::{DisplayID, Rect, WindowID};
use crate::sys::{UnsafeSCShareableContent, UnsafeSCWindow};

pub struct SCRunningApplication {
    unsafe_ptr: Id<UnsafeSCRunningApplication, Shared>,
    process_id: isize,
    bundle_identifier: Option<String>,
    application_name: Option<String>,
}

impl SCRunningApplication {
    fn new(unsafe_ptr: Id<UnsafeSCRunningApplication, Shared>) -> Self {
        let ptr = unsafe_ptr.clone();
        SCRunningApplication {
            process_id:ptr.get_process_id(),
            bundle_identifier: ptr.get_bundle_identifier().map(String::from),
            application_name: ptr.get_application_name().map(String::from),
            unsafe_ptr: ptr,
        }
    }
}

pub struct SCWindow {
    unsafe_ptr: Id<UnsafeSCWindow, Shared>,
    pub title: Option<String>,
    pub owning_application: Option<Id<SCRunningApplication, Shared>>,
    pub window_id: WindowID,
    pub window_layer: u32,
    pub is_active: bool,
    pub is_on_screen: bool,
}
impl SCWindow {
    pub fn new(unsafe_ptr: Id<UnsafeSCWindow, Shared>) -> Self {
        SCWindow {
            unsafe_ptr: unsafe_ptr.clone(),
        }
    }
}
pub struct SCDisplay {
    pub display_id: DisplayID,
    pub frame: Rect,
    pub width: u64,
    pub height: u64,
}

pub struct SCShareableContent {
    unsafe_ptr: Id<UnsafeSCShareableContent, Shared>,
    pub windows: Vec<SCWindow>,
    
   // pub applications: Vec<SCRunningApplication<'a>>,
   // pub displays: Vec<SCDisplay>,
}

impl  SCShareableContent  {
    pub fn current() -> Self {
        let unsafe_ptr = UnsafeSCShareableContent::get().unwrap();
        let windows: Vec<SCWindow> = unsafe_ptr.clone().windows().into_iter().map(SCWindow::new).collect();
        SCShareableContent {
            unsafe_ptr: unsafe_ptr.clone(),
            windows, 
            //applications: todo!(),
            //displays: todo!(),
        }
    }
}
