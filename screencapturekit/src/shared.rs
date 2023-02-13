pub type DisplayID = u32;
pub type WindowID = u32;

macro_rules! get_string {
    // The `expr` designator is used for expressions.
    ($obj:ident, $name: ident) => {{
        let string_ptr: *const NSString = msg_send![$obj, $name];
        if string_ptr.is_null() {
            None
        } else {
            Some((*string_ptr).as_str().to_owned())
        }
    }};
}

pub(crate) use get_string;

#[derive(Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
