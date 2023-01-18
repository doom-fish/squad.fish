use std::{ffi::c_char, slice, thread, time};

use block::ConcreteBlock;
use screencapturekit_sys::{
    INSArray, INSString, ISCShareableContent, ISCWindow, NSError,
    NSString_NSStringExtensionMethods, SCShareableContent, SCWindow,
};

fn main() {
    unsafe {
        let c = |s: SCShareableContent, _: NSError| {
            let w = s.windows();
            let win = SCWindow(INSArray::<SCWindow>::objectAtIndex_(&w, 0));

            let bytes = {
                let bytes: *const c_char = win.title().UTF8String();
                bytes as *const u8
            };
            let len = win.title().length() as usize;
            let st = {
                let bytes = slice::from_raw_parts(bytes, len);
                std::str::from_utf8(bytes).unwrap()
            };

            println!("{}", st);
        };
        let block = ConcreteBlock::new(c).clone();

        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler_(0, 1, &*block)
    }

    let ten_millis = time::Duration::from_millis(10000);

    thread::sleep(ten_millis);
}
