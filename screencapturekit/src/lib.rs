#![allow(dead_code)]
use block::ConcreteBlock;
use objc::{
    runtime::{Class, Object},
    *,
};
use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSString};
use objc_id::Id;
use std::sync::mpsc::{channel, RecvError};

type IdPtr = *mut Object;
type WindowID = u32;
type DisplayID = u32;

struct NSError(IdPtr);

pub struct SCShareableContent(Id<Object>);

pub struct Size {
    width: f64,
    height: f64,
}

pub struct Point {
    x: f64,
    y: f64,
}

pub struct Rect {
    origin: Point,
    size: Size,
}

pub struct SCRunnableApplication<'a> {
    process_id: isize,
    bundle_identifier: &'a str,
    application_name: &'a str,
}

pub struct SCWindow<'a> {
    ptr: Id<Object>,
    pub title: Option<&'a str>,
    pub owning_application: Option<&'a SCRunnableApplication<'a>>,
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
struct RawSCRunnableApplication;
unsafe impl Message for RawSCRunnableApplication {}

impl RawSCRunnableApplication {
    
    fn get_process_id(&self) -> isize {
        unsafe { msg_send![&*self, processID] }
    }

    fn get_application_name(&self) -> Option<&str> {
        unsafe {
            let obj = &*(self as *const _ as *const Object);
            let str_id: IdPtr = msg_send![obj, applicationName];
            if str_id.is_null() {
                None
            } else {
                let str = str_id as *const NSString;
                Some((*str).as_str())
            }
        }
    }
    fn get_bundle_identifier(&self) -> Option<&str> {
        unsafe {
            let obj = &*(self as *const _ as *const Object);
            let str_id: IdPtr = msg_send![obj, bundleIdentifier];
            if str_id.is_null() {
                None
            } else {
                let str = str_id as *const NSString;
                Some((*str).as_str())
            }
        }
    }
}

struct RawSCWindow;
unsafe impl Message for RawSCWindow {}

impl RawSCWindow {
    fn get_owning_application(&self) -> &RawSCRunnableApplication {
        unsafe { msg_send![&*self, owningApplication] }
    }
    fn get_window_layer(&self) -> u32 {
        unsafe { msg_send![&*self, windowLayer] }
    }
    fn get_window_id(&self) -> WindowID {
        unsafe { msg_send![&*self, windowID] }
    }

    fn get_title(&self) -> Option<&str> {
        unsafe {
            let obj = &*(self as *const _ as *const Object);
            let str_id: IdPtr = msg_send![obj, title];
            if str_id.is_null() {
                None
            } else {
                let str = str_id as *const NSString;
                Some((*str).as_str())
            }
        }
    }
}

impl INSObject for RawSCWindow {
    fn class() -> &'static runtime::Class {
        Class::get("SCWindow")
            .expect("Missing SCWindow class, check that the binary is linked with ScreenCaptureKit")
    }
}

impl SCShareableContent {
    pub fn get() -> Result<Self, RecvError> {
        let (tx, rx) = channel();

        let handler = ConcreteBlock::new(move |sc_ptr: IdPtr, _: NSError| {
            let sc_id: Id<Object> = unsafe { Id::from_ptr(sc_ptr) };
            tx.send(SCShareableContent(sc_id)).expect("Should work!");
        });

        unsafe {
            let _: () = msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: &*handler.copy()
            ];
        };

        rx.recv()
    }
    pub fn windows(&self) -> Vec<SCWindow> {
        let win_array_ptr: Id<NSArray<RawSCWindow>> =
            unsafe { Id::from_ptr(msg_send!(self.0, windows)) };
        for (_, w) in win_array_ptr.object_enumerator().enumerate() {
           println!("www {}", w.get_owning_application().get_application_name().unwrap_or("NONE"));
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get() {
        let sc = SCShareableContent::get().unwrap();
        sc.windows();
    }
}
