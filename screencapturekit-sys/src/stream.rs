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
use objc_foundation::{INSObject, INSString, NSObject, NSString};
use objc_id::Id;

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

            tx.send(()).expect("LL");
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
        let queue = Queue::create("fish.doom.screencapturekit", QueueAttribute::Serial);

        let a = UnsafeSCStreamOutputHandler::init(handle);
        unsafe {
            let _: () = msg_send!(self, addStreamOutput: a type: 0 sampleHandlerQueue: queue error: NSObject::new());
        }
    }
}

#[cfg(test)]
mod stream_test {
    use std::sync::mpsc::{sync_channel, SyncSender};

    use super::{UnsafeSCStream, UnsafeSCStreamError};
    use crate::{
        content_filter::{UnsafeContentFilter, UnsafeInitParams::Display},
        shareable_content::UnsafeSCShareableContent,
        stream_configuration::UnsafeStreamConfiguration,
        stream_output_handler::UnsafeSCStreamOutput,
    };
    struct ErrorHandler {}
    #[repr(C)]
    struct OutputHandler {
        tx: SyncSender<()>,
    }
    impl Drop for OutputHandler {
        fn drop(&mut self) {
            println!("DROPPP");
        }
    }
    impl UnsafeSCStreamError for ErrorHandler {
        fn handle_error(&self) {
            eprintln!("ERROR!");
        }
    }
    impl UnsafeSCStreamOutput for OutputHandler {
        fn got_sample(&self) {
            eprintln!("SAMPPLE!");
            self.tx.send(()).unwrap();
        }
    }
    #[test]
    fn test_sc_stream() {
        let display = UnsafeSCShareableContent::get()
            .unwrap()
            .displays()
            .pop()
            .expect("could not get display");

        let filter = UnsafeContentFilter::init(Display(display));
        let config = UnsafeStreamConfiguration {
            width: 100,
            height: 100,
            ..Default::default()
        };
        let (tx, rx) = sync_channel(1);
        let stream = UnsafeSCStream::init(filter, config.into(), ErrorHandler {});
        let a = OutputHandler { tx };
        stream.add_stream_output(a);
        stream.start_capture();
        rx.recv().unwrap();
    }
}
