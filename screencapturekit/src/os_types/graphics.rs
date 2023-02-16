#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]

use super::geometry::CGFloat;

#[derive(Debug)]
pub struct CGColor;
unsafe impl Message for CGColor {}

impl INSObject for CGColor {
    fn class() -> &'static runtime::Class {
        Class::get("CGColor")
            .expect("Missing CGColor class, check that the binary is linked with CoreGraphics")
    }
}


impl CGColor {
    pub fn rgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Id<Self> {
        unsafe {
            let ptr = msg_send![CGColorCreateGenericRGB, red, green, blue, alpha];
            Id::from_ptr(ptr)
        }
    }
}
