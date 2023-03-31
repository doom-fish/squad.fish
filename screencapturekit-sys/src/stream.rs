use std::{
    pin::Pin,
    ptr::addr_of,
    sync::{
        mpsc::{channel, Receiver},
        Once,
    },
};

use block::{ConcreteBlock, RcBlock};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Sel},
    Encode, Encoding, Message, *,
};

use super::{
    content_filter::UnsafeContentFilter, stream_configuration::UnsafeStreamConfigurationRef,
};
use dispatch::{Queue, QueueAttribute};
use objc_foundation::{INSObject, INSString, NSObject, NSString};
use objc_id::{Id, ShareId};

pub trait UnsafeSCStreamError {
    //fn handle_error(&self, stream: &UnsafeSCStream, error: u8) {
    //  eprintln!("ERROR!");
    //}
}

pub trait UnsafeSCStreamOutput {
    //  fn got_sample(&self, stream: &UnsafeSCStream, sample: UnsafeSCSample, type: OutputType);
}

#[repr(C)]
pub struct UnsafeSCStreamHandle {}

unsafe impl Message for UnsafeSCStreamHandle {}

impl INSObject for UnsafeSCStreamHandle {
    fn class() -> &'static Class {
        static REGISTER_UNSAFE_SC_STREAM: Once = Once::new();
        REGISTER_UNSAFE_SC_STREAM.call_once(|| {
            let mut decl = ClassDecl::new("SCStreamHandle", class!(NSObject)).unwrap();
            decl.add_ivar::<usize>("_output_handler");
            decl.add_ivar::<usize>("_error_handler");

            extern "C" fn stream_error(
                this: &mut Object,
                _cmd: Sel,
                stream: *mut Object,
                error: *mut Object,
            ) {
                unsafe {
                    let ptr = *this.get_ivar::<usize>("_error_handler");
                    let error_handler = addr_of!(ptr) as *const Box<&dyn UnsafeSCStreamError>;
                    if !error_handler.is_null() {
                        // (*error_handler).handle_error();
                    }
                };
            }
            extern "C" fn stream_sample(
                _this: &mut Object,
                _cmd: Sel,
                _stream: *mut Object,
                _sample: *mut Object,
                _type: u8,
            ) {
            }
            unsafe {
                let stream_error_method: extern "C" fn(&mut Object, Sel, *mut Object, *mut Object) =
                    stream_error;

                let stream_sample_method: extern "C" fn(
                    &mut Object,
                    Sel,
                    *mut Object,
                    *mut Object,
                    u8,
                ) = stream_sample;
                decl.add_method(sel!(stream:didStopWithError:), stream_error_method);
                decl.add_method(
                    sel!(stream:didOutputSampleBuffer:ofType:),
                    stream_sample_method,
                );
            }

            decl.register();
        });
        class!(SCStreamHandle)
    }
}

impl UnsafeSCStreamHandle {
    fn store_output_handler(&mut self, dyn_trait: &dyn UnsafeSCStreamOutput) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let trait_ptr = Box::into_raw(Box::new(dyn_trait));

            obj.set_ivar("_output_handler", trait_ptr as usize);
        }
    }
    fn store_error_handler(&mut self, dyn_trait: &dyn UnsafeSCStreamError) {
        unsafe {
            let obj = &mut *(self as *mut _ as *mut Object);
            let trait_ptr = Box::into_raw(Box::new(dyn_trait));

            obj.set_ivar("_error_handler", trait_ptr as usize);
        }
    }
    pub fn init(
        error_handler: Option<impl UnsafeSCStreamError>,
        output_handler: Option<impl UnsafeSCStreamOutput>,
    ) -> Id<Self> {
        let mut handle = Self::new();
        if let Some(output_handler) = output_handler {
            handle.store_output_handler(&output_handler);
        }
        if let Some(error_handler) = error_handler {
            handle.store_error_handler(&error_handler);
        }
        handle
    }
}

#[derive(Debug)]
pub struct UnsafeSCStream;
unsafe impl Message for UnsafeSCStream {}
impl INSObject for UnsafeSCStream {
    fn class() -> &'static Class {
        Class::get("SCStream")
            .expect("Missing SCStream class, check that the binary is linked with ScreenCaptureKit")
    }
}
type CompletionHandlerBlock = RcBlock<(*mut Object,), ()>;
impl UnsafeSCStream {
    unsafe fn new_completion_handler() -> (CompletionHandlerBlock, Receiver<()>) {
        let (tx, rx) = channel();
        let handler = ConcreteBlock::new(move |error: *mut Object| {
            if !error.is_null() {
                let code: *mut NSString = msg_send![error, localizedDescription];
                eprintln!("{:?}", (*code).as_str());
                panic!("start fail");
            }

            tx.send(()).expect("LALALA");
        });
        (handler.copy(), rx)
    }

    pub fn init(
        filter: Id<UnsafeContentFilter>,
        config: Id<UnsafeStreamConfigurationRef>,
        handle: ShareId<UnsafeSCStreamHandle>,
    ) -> Id<Self> {
        let instance = UnsafeSCStream::new();
        unsafe {
            let _: () =
                msg_send![instance, initWithFilter: filter  configuration: config delegate: handle];
        }
        instance
    }
    pub fn start_capture(&self) {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            let _: () = msg_send!(self, startCaptureWithCompletionHandler: handler);
            rx.recv().expect("LALAL");
        }
    }
    pub fn add_stream_output(&self, handle: ShareId<UnsafeSCStreamHandle>) {
        unsafe {
            let queue = Queue::create("fish.doom.screencapturekit", QueueAttribute::Serial);
            let _: () = msg_send!(self, addStreamOutput: &*handle type: 0 sampleHandlerQueue: queue error: NSObject::new());
        }
    }
}

#[cfg(test)]
mod stream_test {
    use std::{ptr, thread, time};

    use objc::{msg_send, runtime::Object, *};
    use objc_foundation::INSObject;

    use crate::{
        content_filter::{UnsafeContentFilter, UnsafeContentFilterInitParams},
        shareable_content::UnsafeSCShareableContent,
        stream_configuration::UnsafeStreamConfiguration,
    };

    use super::{UnsafeSCStream, UnsafeSCStreamError, UnsafeSCStreamHandle, UnsafeSCStreamOutput};

    #[repr(C)]
    struct TestHandler {}
    impl UnsafeSCStreamError for TestHandler {}
    impl UnsafeSCStreamOutput for TestHandler {}

    //    #[test]
    fn test_sc_stream() {
        let display = UnsafeSCShareableContent::get()
            .unwrap()
            .displays()
            .pop()
            .unwrap();
        let params = UnsafeContentFilterInitParams::Display(display);
        let filter = UnsafeContentFilter::init(params);

        let config = UnsafeStreamConfiguration {
            width: 100,
            height: 100,
            ..Default::default()
        };

        let handle = UnsafeSCStreamHandle::new().share();

        let stream = UnsafeSCStream::init(filter, config.into(), handle.clone());
        stream.add_stream_output(handle);
        stream.start_capture();
        thread::sleep(time::Duration::from_millis(10_000));
    }
    #[test]
    fn test_sc_stream_handle() {
        let handle = UnsafeSCStreamHandle::init(Some(TestHandler {}), Some(TestHandler {}));
        unsafe {
            msg_send![handle, stream: ptr::null_mut() as *mut Object didStopWithError: ptr::null_mut() as *mut Object]
        }

        unsafe {
            // msg_send![handle, stream: ptr::null_mut() as *mut Object didOutputSampleBuffer: ptr::null_mut() as *mut Object ofType: 0]
        };
    }
}
