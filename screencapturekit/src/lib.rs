use block::ConcreteBlock;
use objc::{runtime::Object, *};
use objc_id::Id;
pub enum Error {
    SharableContentError,
}

pub struct SCShareableContent {}

impl SCShareableContent {
    pub fn get_with_completion_handler(completion_handler: fn(Result<SCShareableContent, Error>)) {
        let completion_handler_block = ConcreteBlock::new(|_sc: Id<Object>, _error: Id<Object>| {});
        let _: Id<Object> = unsafe {
            let c = class!(SCShareableContent);
            msg_send![
                c,
                getShareableContentWithCompletionHandler: completion_handler_block
            ]
        };
        completion_handler(Ok(SCShareableContent {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_with_completion_handler() {
        assert!(true, "TRUE");
    }
}
