use std::sync::Once;

use objc::{
    declare::ClassDecl,
    runtime::{Class, Object, Protocol, Sel},
    Message, *,
};

use objc_foundation::INSObject;
use objc_id::Id;

use super::{stream_configuration::UnsafeStreamConfiguration, content_filter::UnsafeContentFilter};

#[derive(Debug)]
struct SCStreamHandle;
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

            extern "C" fn stream(_this: &mut Object, _cmd: Sel, test_num: u32) {
                println!("{:?}", test_num);
            }
            unsafe {
                let protocol_stream: extern "C" fn(&mut Object, Sel, u32) = stream;
                decl.add_method(sel!(stream:), protocol_stream);
            }

            decl.register();
        });
        class!(SCStreamHandle)
    }
}

struct UnsafeSCStream;
unsafe impl Message for UnsafeSCStream {}
impl INSObject for UnsafeSCStream {
    fn class() -> &'static Class {
        Class::get("SCStream")
            .expect("Missing SCStream class, check that the binary is linked with ScreenCaptureKit")
    }
}
impl UnsafeSCStream {
    fn init(filter: Id<UnsafeContentFilter, config: Id<UnsafeStreamConfiguration>, handle: Id<SCStreamHandle>) {
        let instance = UnsafeSCStream::new();
        unsafe { let _:() = msg_send![instance, init: ]  } 
    }
}

#[cfg(test)]
mod stream_test {
    use objc::{msg_send, runtime::Protocol, *};
    use objc_foundation::INSObject;

    use super::SCStreamHandle;
    #[test]
    fn test_constructor() {
        let handle = SCStreamHandle::new();
        unsafe { msg_send![handle, stream: 4] }
    }
    #[test]
    fn test_protocols_exists() {
        Protocol::get("SCStreamOutput").expect("Should Exist");
        Protocol::get("SCStreamDelegate").expect("Should exist");
    }
}
