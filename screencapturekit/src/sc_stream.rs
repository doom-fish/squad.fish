use screencapturekit_sys::{
    os_types::rc::Id, stream::UnsafeSCStream, stream_error_handler::UnsafeSCStreamError,
    stream_output_handler::UnsafeSCStreamOutput,
};

use crate::{
    sc_content_filter::SCContentFilter,
    sc_error_handler::{StreamErrorHandler, StreamErrorHandlerWrapper},
    sc_stream_configuration::SCStreamConfiguration,
};

#[derive(Debug)]
pub struct SCStream {
    pub(crate) _unsafe_ref: Id<UnsafeSCStream>,
}

impl SCStream {
    pub fn new(
        filter: SCContentFilter,
        config: SCStreamConfiguration,
        handler: impl StreamErrorHandler,
    ) -> Self {
        Self {
            _unsafe_ref: UnsafeSCStream::init(
                filter._unsafe_ref,
                config._unsafe_ref,
                StreamErrorHandlerWrapper::new(handler),
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{sc_output_handler::StreamOutput, sc_shareable_content::SCShareableContent};

    use super::*;
    struct SomeOutputWrapper {}
    impl StreamOutput for SomeOutputWrapper {
        fn stream_output(&self) {}
    }
    #[test]
    fn test_output_wrapper() {
        let content = SCShareableContent::current();
        let window = content.windows.first();
        let filter = SCContentFilter
        let tream = SCStream::new();
    }
}
