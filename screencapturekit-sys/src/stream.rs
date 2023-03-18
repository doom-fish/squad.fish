use std::{
    ffi::CString,
    sync::{
        mpsc::{channel, Receiver},
        Once,
    },
};

use block::{ConcreteBlock, RcBlock};
use objc::{
    declare::{ClassDecl, ProtocolDecl},
    runtime::{Class, Object, Protocol, Sel},
    Message, *,
};

use objc_foundation::{INSObject, INSString, NSObject, NSString};
use objc_id::{Id, ShareId};

use super::{
    content_filter::UnsafeContentFilter, stream_configuration::UnsafeStreamConfigurationRef,
};
use dispatch::{Queue, QueueAttribute};

pub trait UnsafeSCStreamError {}
pub trait UnsafeSCStreamOutput {}

pub struct UnsafeSCStreamHandle {
    output: Box<dyn UnsafeSCStreamOutput>,
    error: Box<dyn UnsafeSCStreamError>,
}

unsafe impl Message for UnsafeSCStreamHandle {}
pub fn output_protocol() -> &'static Protocol {
    static REGISTER_CUSTOM_PROTOCOL: Once = Once::new();

    REGISTER_CUSTOM_PROTOCOL.call_once(|| {
        let mut decl = ProtocolDecl::new("StreamOutput").unwrap();

        decl.add_method_description::<(*mut Object, *mut Object, u8), ()>(sel!(sstream:didOutputSampleBuffer:ofType:), false);

        decl.register();
    });

    Protocol::get("StreamOutput").unwrap()
}
pub fn delegate_protocol() -> &'static Protocol {
    static REGISTER_CUSTOM_PROTOCOL: Once = Once::new();

    REGISTER_CUSTOM_PROTOCOL.call_once(|| {
        let mut decl = ProtocolDecl::new("StreamDelegate").unwrap();

        decl.add_method_description::<(*mut Object, *mut Object), ()>(sel!(stream:didStopWithError:), false);

        decl.register();
    });

    Protocol::get("StreamDelegate").unwrap()
}
impl INSObject for UnsafeSCStreamHandle {
    fn class() -> &'static Class {
        static REGISTER_UNSAFE_SC_STREAM: Once = Once::new();
        REGISTER_UNSAFE_SC_STREAM.call_once(|| {
            let scstream_delegate = delegate_protocol(); 
            let scstream_output = output_protocol();
            let mut decl = ClassDecl::new("SCStreamHandle", class!(NSObject)).unwrap();

            decl.add_protocol(scstream_delegate);
            decl.add_protocol(scstream_output);

            extern "C" fn stream_error(
                _this: &mut Object,
                _cmd: Sel,
                _stream: *mut Object,
                _error: *mut Object,
            ) {
                println!("GOT error");
            }
            extern "C" fn stream_sample(
                this_: &mut Object,
                _cmd: Sel,
                _stream: *mut Object,
                _sample: *mut Object,
                _type: u8,
            ) {
                let sc = unsafe { this_ };
                println!("GOT SAMPLE");
            }
            unsafe {
                let stream_error_method: extern "C" fn(
                    &mut Object,
                    Sel,
                    *mut Object,
                    *mut Object,
                ) = stream_error;

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
    pub fn init() -> Id<Self> {
        let mut handle = Self::new();
        // handle.output = output;
        // handle.error = error;
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

    use objc::{
        msg_send,
        runtime::{Object, Protocol},
        *,
    };
    use objc_foundation::INSObject;

    use crate::{
        content_filter::{UnsafeContentFilter, UnsafeContentFilterInitParams},
        shareable_content::UnsafeSCShareableContent,
        stream_configuration::UnsafeStreamConfiguration,
    };

    use super::{UnsafeSCStream, UnsafeSCStreamHandle};
    #[test]
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
        let handle = UnsafeSCStreamHandle::new();
        unsafe { msg_send![handle, stream: ptr::null_mut() as *mut Object didStopWithError: ptr::null_mut() as *mut Object] }
        unsafe {
            msg_send![handle, stream: ptr::null_mut() as *mut Object didOutputSampleBuffer: ptr::null_mut() as *mut Object ofType: 0]
        }
    }
    #[test]
    fn test_protocols_exists() {
        Protocol::get("SCStreamOutput").expect("Should Exist");
        Protocol::get("SCStreamDelegate").expect("Should exist");
    }
}
