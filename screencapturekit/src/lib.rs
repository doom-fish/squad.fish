use block::ConcreteBlock;
use objc::{runtime::Object, *};
use objc_id::Id;
use std::sync::mpsc::{channel, RecvError};

type IdPtr = *mut Object;

struct NSError(IdPtr);

pub struct SCShareableContent(Id<Object>);

pub struct SCWindow<'a> {
    ptr: Id<Object>,
    pub name: &'a str,
    pub id: usize,
}

impl SCShareableContent {
    pub fn get() -> Result<Self, RecvError> {
        let (tx, rx) = channel();

        let handler = ConcreteBlock::new(|sc_ptr: IdPtr, _: NSError| {
            let sc_id: Id<Object> = unsafe { Id::from_ptr(sc_ptr) };
            tx.send(SCShareableContent(sc_id)).expect("Should work!");
        });

        unsafe {
            let _: () = msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: &*handler.clone()
            ];
        };
        rx.recv()
    }
    pub fn windows(&self) -> Vec<SCWindow> {
        unsafe {
            vec![SCWPickersActionBarindow(Id::from_ptr(msg_send!(
                self.0, windows
            )))]
        }
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
