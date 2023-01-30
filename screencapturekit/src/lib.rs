use objc::{runtime::Object, *};
use objc_id::Id;

pub enum Error {
    SharableContentError,
}

pub struct SCShareableContent {}

impl SCShareableContent {
    pub fn get_with_compeletion_handler() {
        let _: Id<Object> = unsafe { msg_send![class!(SCShareableContent), alloc] };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_with_completion_handler() {
        SCShareableContent::get_with_compeletion_handler();
    }
}
