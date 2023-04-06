use std::sync::mpsc::{channel, Receiver};

use block::{ConcreteBlock, RcBlock};
use objc::{
    runtime::{Class, Object},
    Message, *,
};

use crate::{
    stream_error_handler::{UnsafeSCStreamError, UnsafeSCStreamErrorHandler},
    stream_output_handler::{UnsafeSCStreamOutput, UnsafeSCStreamOutputHandler},
};

use super::{
    content_filter::UnsafeContentFilter, stream_configuration::UnsafeStreamConfigurationRef,
};
use dispatch::{Queue, QueueAttribute};
use objc_foundation::{INSObject, NSObject};
use objc_id::Id;

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
                //  let code: *mut NSString = msg_send![error, localizedDescription];
                //eprintln!("{:?}", (*code).as_str());
                panic!("start fail");
            }

            tx.send(()).expect("LALALA");
        });
        (handler.copy(), rx)
    }

    pub fn init(
        filter: Id<UnsafeContentFilter>,
        config: Id<UnsafeStreamConfigurationRef>,
        error_handler: impl UnsafeSCStreamError,
    ) -> Id<Self> {
        let instance = UnsafeSCStream::new();
        unsafe {
            let _: () = msg_send![instance, initWithFilter: filter  configuration: config delegate: UnsafeSCStreamErrorHandler::init(error_handler)];
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
    pub fn add_stream_output(&self, handle: impl UnsafeSCStreamOutput) {
        unsafe {
            let queue = Queue::create("fish.doom.screencapturekit", QueueAttribute::Serial);
            let _: () = msg_send!(self, addStreamOutput: UnsafeSCStreamOutputHandler::init(handle) type: 0 sampleHandlerQueue: queue error: NSObject::new());
        }
    }
}

#[cfg(test)]
mod stream_test {
    use std::{thread, time};

    use crate::{
        content_filter::{UnsafeContentFilter, UnsafeContentFilterInitParams},
        shareable_content::UnsafeSCShareableContent,
        stream_configuration::UnsafeStreamConfiguration,
        stream_output_handler::UnsafeSCStreamOutput,
    };

    use super::{UnsafeSCStream, UnsafeSCStreamError};

    #[repr(C)]
    struct TestHandler {}
    impl UnsafeSCStreamError for TestHandler {
        fn handle_error(&self) {
            eprintln!("ERROR!");
        }
    }
    impl UnsafeSCStreamOutput for TestHandler {
        fn got_sample(&self) {
            eprintln!("SAMPPLE!");
        }
    }
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

        let stream = UnsafeSCStream::init(filter, config.into(), TestHandler {});
        stream.add_stream_output(TestHandler {});
        stream.start_capture();
        thread::sleep(time::Duration::from_millis(10_000));
    }
}
