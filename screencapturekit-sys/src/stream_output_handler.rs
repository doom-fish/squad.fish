use std::{
    collections::HashMap,
    sync::{Once, RwLock},
};

use objc::{
    class,
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    Message, *,
};
use objc_foundation::INSObject;
use objc_id::Id;
use once_cell::sync::Lazy;

static OUTPUTS: Lazy<RwLock<HashMap<usize, Box<dyn UnsafeSCStreamOutput + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[repr(C)]
pub(crate) struct UnsafeSCStreamOutputHandler {}

impl Drop for UnsafeSCStreamOutputHandler {
    fn drop(&mut self) {
        println!("dropped");
    }
}

pub trait UnsafeSCStreamOutput: Send + Sync + 'static {
    fn got_sample(&self);
}

unsafe impl Message for UnsafeSCStreamOutputHandler {}

impl INSObject for UnsafeSCStreamOutputHandler {
    fn class() -> &'static Class {
        static REGISTER_UNSAFE_SC_OUTPUT_HANDLER: Once = Once::new();
        REGISTER_UNSAFE_SC_OUTPUT_HANDLER.call_once(|| {
            let mut decl = ClassDecl::new("SCStreamOutputHandler", class!(NSObject)).unwrap();
            decl.add_ivar::<usize>("_trait");

            extern "C" fn stream_output(
                this: &mut Object,
                _cmd: Sel,
                _stream: *mut Object,
                _sample: *mut Object,
                _error: *mut Object,
            ) {
                unsafe {
                    let ptr = this.get_ivar::<usize>("_trait");
                    let h = OUTPUTS.read().unwrap();
                    h.get(ptr).unwrap().got_sample();
                };
            }
            unsafe {
                let stream_output_method: extern "C" fn(
                    &mut Object,
                    Sel,
                    *mut Object,
                    *mut Object,
                    *mut Object,
                ) = stream_output;

                decl.add_method(
                    sel!(stream:didOutputSampleBuffer:ofType:),
                    stream_output_method,
                );
            }

            decl.register();
        });
        class!(SCStreamOutputHandler)
    }
}

impl UnsafeSCStreamOutputHandler {
    fn store_output_handler(&mut self, output_handler: impl UnsafeSCStreamOutput) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let hash = self.hash_code();
            OUTPUTS
                .write()
                .unwrap()
                .insert(hash, Box::new(output_handler));
            obj.set_ivar("_trait", hash);
        }
    }
    pub fn init(output_handler: impl UnsafeSCStreamOutput) -> Id<Self> {
        let mut handle = Self::new();
        handle.store_output_handler(output_handler);
        handle
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    #[repr(C)]
    struct TestHandler {}
    impl UnsafeSCStreamOutput for TestHandler {
        fn got_sample(&self) {
            println!("GOT SAMPLE!");
        }
    }

    #[test]
    fn test_sc_stream_output_handler() {
        let handle = TestHandler {};
        let handle = UnsafeSCStreamOutputHandler::init(handle);
        let _: () = unsafe {
            msg_send![handle, stream: ptr::null_mut() as *mut Object didOutputSampleBuffer: ptr::null_mut() as *mut Object ofType: 0]
        };
    }
}
