use block::ConcreteBlock;
use objc::{runtime::Object, *};
use objc_id::Id;
use std::sync::mpsc::{channel, RecvError};

type IdPtr = *mut Object;

struct NSError(IdPtr);

pub struct SCShareableContent(Id<Object>);

impl SCShareableContent {
    pub fn get() -> Result<Self, RecvError> {
        let (tx, rx) = channel();

        let handler = ConcreteBlock::new(|sc_ptr: IdPtr, _: NSError| {
            let sc_id: Id<Object> = unsafe { Id::from_ptr(sc_ptr) };
            tx.send(SCShareableContent(sc_id)).expect("Should work!");
        });

        let _: () = unsafe {
            msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: &*handler.clone()
            ]
        };
        rx.recv()
    }
    pub fn windows(&self) -> IdPtr {
        unsafe { msg_send!(self.0, windows) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get() {
       let sc = SCShareableContent::get().unwrap();
        sc.windows();
    }
}
