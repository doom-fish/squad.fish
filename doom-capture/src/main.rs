use screencapturekit_sys::{ISCStream, SCStream};

fn main() {
    let stream = SCStream::alloc();
    unsafe { stream.init() };
}
