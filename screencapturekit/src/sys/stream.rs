
use objc::{
    runtime::Class,
    Message, 
};

use objc_foundation::INSObject;


#[derive(Debug)]
struct UnsafeStream;
unsafe impl Message for UnsafeStream {}

impl UnsafeStream {}

impl INSObject for UnsafeStream {
    fn class() -> &'static Class {
        Class::get("SCStream")
            .expect("Missing SCStream class, check that the binary is linked with ScreenCaptureKit")
    }
}

#[cfg(test)]
mod stream_test {
    use objc::runtime::Protocol;

    #[test]
    fn test_protocols_exists() {
        Protocol::get("SCStreamOutput").expect("Should Exist");
        Protocol::get("SCStreamDelegate").expect("Should exist");
    }
} 
