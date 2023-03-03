use std::{
    ptr,
    sync::{
        mpsc::{channel, Receiver},
        Once,
    },
};

use block::{ConcreteBlock, RcBlock};
use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Protocol, Sel},
    Message, *,
};

use objc_foundation::{INSObject, INSString, NSObject, NSString};
use objc_id::{Id, ShareId};

use super::{content_filter::UnsafeContentFilter, stream_configuration::UnsafeStreamConfiguration};

#[derive(Debug)]
pub struct SCStreamHandle;

unsafe impl Message for SCStreamHandle {}

impl INSObject for SCStreamHandle {
    fn class() -> &'static Class {
        static REGISTER_UNSAFE_SC_STREAM: Once = Once::new();
        REGISTER_UNSAFE_SC_STREAM.call_once(|| {
            // The runtime will call this method, so it has to be implemented
            extern "C" fn class_init(_this: &Class, _cmd: Sel) {}

            let scstream_output = Protocol::get("SCStreamOutput").expect("Should Exist");
            let scstream_delegate = Protocol::get("SCStreamDelegate").expect("Should exist");
            let mut decl = ClassDecl::new("SCStreamHandle", class!(NSObject)).unwrap();

            decl.add_protocol(scstream_delegate);
            decl.add_protocol(scstream_output);

            extern "C" fn stream(_this: &mut Object, _cmd: Sel, sample: *mut Object) {
                println!("GOT SAMPLE");
            }
            unsafe {
                let protocol_stream: extern "C" fn(&mut Object, Sel, *mut Object) = stream;
                decl.add_method(sel!(stream:), protocol_stream);
            }

            decl.register();
        });
        class!(SCStreamHandle)
    }
}

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
        config: Id<UnsafeStreamConfiguration>,
        handle: ShareId<SCStreamHandle>,
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
    pub fn add_stream_output(&self, handle: ShareId<SCStreamHandle>) {
        unsafe {
            let nil: *mut NSObject = ptr::null_mut();
            let _: () = msg_send!(self, addStreamOutput: handle type: 0 sampleHandlerQueue: nil  error: nil);
        }
    }
}

#[cfg(test)]
mod stream_test {
    use std::{thread, time};

    use objc::{msg_send, runtime::Protocol, *};
    use objc_foundation::INSObject;

    use crate::sys::{
        content_filter::{InitParams, UnsafeContentFilter},
        shareable_content::UnsafeSCShareableContent,
        stream_configuration::SCStreamConfiguration,
    };

    use super::{SCStreamHandle, UnsafeSCStream};
    #[test]
    fn test_sc_stream() {
        let display = UnsafeSCShareableContent::get()
            .unwrap()
            .displays()
            .pop()
            .unwrap();
        let params = InitParams::Display(display);
        let filter = UnsafeContentFilter::init(params);

        let mut config = SCStreamConfiguration::default();
        config.width = 100;
        config.height = 100;
        let handle = SCStreamHandle::new().share();

        let stream = UnsafeSCStream::init(filter, config.into(), handle.clone());
        stream.add_stream_output(handle);
        stream.start_capture();
        thread::sleep(time::Duration::from_millis(10_000));
    }
    #[test]
    fn test_sc_stream_handle() {
        let handle = SCStreamHandle::new();
        unsafe { msg_send![handle, stream: 4] }
    }
    #[test]
    fn test_protocols_exists() {
        Protocol::get("SCStreamOutput").expect("Should Exist");
        Protocol::get("SCStreamDelegate").expect("Should exist");
    }
}
