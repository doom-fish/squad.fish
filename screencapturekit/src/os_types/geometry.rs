
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]

pub type CGFloat = ::std::ffi::c_double;
pub type CGError = ::std::ffi::c_int;

pub const kCGErrorSuccess: CGError = 0;
pub const kCGErrorFailure: CGError = 1000;
pub const kCGErrorIllegalArgument: CGError = 1001;
pub const kCGErrorInvalidConnection: CGError = 1002;
pub const kCGErrorInvalidContext: CGError = 1003;
pub const kCGErrorCannotComplete: CGError = 1004;
pub const kCGErrorNotImplemented: CGError = 1006;
pub const kCGErrorRangeCheck: CGError = 1007;
pub const kCGErrorTypeCheck: CGError = 1008;
pub const kCGErrorInvalidOperation: CGError = 1010;
pub const kCGErrorNoneAvailable: CGError = 1011;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    #[inline]
    pub fn new(width: CGFloat, height: CGFloat) -> CGSize {
        CGSize {
            width,
            height,
        }
    }

}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    #[inline]
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint {
        CGPoint { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    #[inline]
    pub fn new(origin: &CGPoint, size: &CGSize) -> CGRect {
        CGRect {
            origin: *origin,
            size: *size,
        }
    }
}
