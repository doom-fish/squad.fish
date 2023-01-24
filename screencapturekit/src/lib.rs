
use objc::{msg_send};

pub enum Error {
    SharableContentError,
}

pub struct SCShareableContent {}

impl SCShareableContent {
    pub fn get_with_completion_handler(completion_handler: fn(Result<SCShareableContent, Error>)) {
        completion_handler(Ok(SCShareableContent {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        SCShareableContent::get_with_completion_handler(|_| print!("TEST"))
    }
}
