use std::sync::{
    mpsc::{channel, Receiver},
    Once,
};

use block::{ConcreteBlock, RcBlock};
use objc::{
    declare::{ClassDecl, ProtocolDecl},
    msg_send,
    runtime::{Class, Object, Protocol, Sel},
    Message, *,
};
use objc_foundation::INSObject;
use objc_id::Id;

use super::{content_filter::UnsafeContentFilter, stream_configuration::UnsafeStreamConfiguration};

#[derive(Debug)]
struct UnsafeStream;
unsafe impl Message for UnsafeStream {}

type CompletionHandlerBlock = RcBlock<(*mut Object,), ()>;
impl UnsafeStream {
    unsafe fn new_completion_handler() -> (CompletionHandlerBlock, Receiver<()>) {
        let (tx, rx) = channel();
        let handler = ConcreteBlock::new(move |error: *mut Object| {
            println!("{:?}", error.is_null());
            tx.send(());
        });
        (handler.copy(), rx)
    }

    fn init(
        &self,
        filter: Id<UnsafeContentFilter>,
        config: Id<UnsafeStreamConfiguration>,
        delegate: UnsafeStreamDelegate,
    ) -> Id<Self> {
        unsafe {
            let _: () = msg_send ! (self , initWithFilter : filter configuration : config delegate : delegate);
            Id::from_ptr(self as *const _ as *mut UnsafeStream)
        }
    }
    fn startCapture(&self) {
        unsafe {
            let (handler, rx) = Self::new_completion_handler();
            let _: () = msg_send!(self, startCaptureWithCompletionHandler: handler);
            rx.recv();
        }
    }
}

impl INSObject for UnsafeStream {
    fn class() -> &'static Class {
        Class::get("SCStream")
            .expect("Missing SCStream class, check that the binary is linked with ScreenCaptureKit")
    }
}
#[repr(C)]
struct UnsafeStreamDelegate {}

impl UnsafeStreamDelegate {
    #[no_mangle]
    extern "C" fn stream(a: *mut Object, c: *mut Object) {
        println!("ERROR");
    }
}

#[cfg(test)]
mod stream_test {
    use std::{
        ffi::CString,
        fmt::{self, Debug},
    };

    use objc::{
        class,
        declare::ProtocolDecl,
        runtime::{self, Protocol},
    };
    use objc_foundation::INSObject;

    use crate::sys::{
        content_filter::UnsafeContentFilter, stream_configuration::UnsafeStreamConfiguration,
    };
    use objc::{
        msg_send,
        runtime::{Class, Object, Sel},
        Message, *,
    };

    use super::{UnsafeStream, UnsafeStreamDelegate};
    #[link(name = "SCKITFIX", kind = "framework")]
    extern "C" {}
    #[test]
    fn test_stream() {
        unsafe {
            let obj: Object = msg_send![class!(ImplementProtocols), alloc];
            let mut p: Vec<&str> = Protocol::protocols().iter().map(|p| p.name()).collect();
            p.sort();
            p.iter().for_each(|p| println!("{:?}", p));
        }
        //        Protocol::protocols().into_iter().for_each(|f| println!("{:?}", f));
        let ss = UnsafeStream::new();
        let v = ss.init(
            UnsafeContentFilter::new(),
            UnsafeStreamConfiguration::new(),
            UnsafeStreamDelegate {},
        );
        v.startCapture();
    }
}
