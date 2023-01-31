use block::ConcreteBlock;
use objc::{runtime::Object, *};
use objc_id::Id;
use std::sync::mpsc::{channel, RecvError};

type id = *mut Object;

struct NSError(id);

pub struct SCShareableContent(Id<Object>);

impl SCShareableContent {
    pub fn get() -> Result<Self, RecvError> {
        let (tx, rx) = channel();

        let handler = ConcreteBlock::new(|sc_ptr: id, _: NSError| {
            let sc_id: Id<Object> = unsafe { Id::from_ptr(sc_ptr) };
            tx.send(SCShareableContent(sc_id)).expect("Should work!");
        });

        #[allow(clippy::redundant_clone)]
        let raw_handler = &*handler.clone();
        let _: () = unsafe {
            msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: raw_handler
            ]
        };
        rx.recv()
    }
    pub fn windows(&self) -> id {
        unsafe { msg_send!(self.0, windows) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_with_completion_handler() {
       let sc = SCShareableContent::get().unwrap();
        sc.windows();
    }
}
