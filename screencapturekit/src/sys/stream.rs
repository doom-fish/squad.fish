#[derive(Debug)]
pub struct UnsafeStream;
unsafe impl Message for UnsafeStream {}
impl UnsafeStream {}

impl INSObject for UnsafeStream {
    fn class() -> &'static Class {
        Class::get("SCStream")
            .expect("Missing SCStream class, check that the binary is linked with ScreenCaptureKit")
    }
}

impl UnsafeStream {
    fn init(&self, Unsafe)
}
