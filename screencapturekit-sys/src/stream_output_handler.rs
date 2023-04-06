use std::{ptr::addr_of, sync::Once};

use objc::{
    class,
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    Message, *,
};
use objc_foundation::INSObject;
use objc_id::Id;

#[repr(C)]
pub(crate) struct UnsafeSCStreamOutputHandler {}

pub trait UnsafeSCStreamOutput {
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
                    let ptr = *this.get_ivar::<usize>("_trait");
                    let stream_output = addr_of!(ptr) as *const Box<&dyn UnsafeSCStreamOutput>;
                    if !stream_output.is_null() {
                        (*stream_output).got_sample();
                    }
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
    fn store_output_handler(&mut self, output_handler: &dyn UnsafeSCStreamOutput) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let trait_ptr = Box::into_raw(Box::new(output_handler));

            obj.set_ivar("_trait", trait_ptr as usize);
        }
    }
    pub fn init(output_handler: impl UnsafeSCStreamOutput) -> Id<Self> {
        let mut handle = Self::new();
        handle.store_output_handler(&output_handler);
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
        let handle = UnsafeSCStreamOutputHandler::init(TestHandler {});
        let _: () = unsafe {
            msg_send![handle, stream: ptr::null_mut() as *mut Object didOutputSampleBuffer: ptr::null_mut() as *mut Object ofType: 0]
        };
    }
}
