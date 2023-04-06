use screencapturekit_sys::{
    os_types::rc::Id, stream::UnsafeSCStream, stream_output_handler::UnsafeSCStreamOutput,
};

pub trait StreamOutput {
    fn stream_output(&self);
}

pub trait StreamErrorHandler {
    fn on_error(&self);
}

#[derive(Debug)]
pub struct SCStream<TO: StreamOutput, TE: StreamErrorHandler> {
    pub(crate) _unsafe_ref: Id<UnsafeSCStream>,
    output: StreamOutputWrapper<TO>,
    error_handler: StreamErrorHandlerWrapper<TE>,
}

impl<TO: StreamOutput, TE: StreamErrorHandler> SCStream<TO, TE> {
    fn new() -> Self {
        todo!();
        // let _unsafe_ref = UnsafeSCStream::init(filter, config, todo!());
    }
}

#[derive(Debug)]
pub struct StreamErrorHandlerWrapper<TErrorHandler>
where
    TErrorHandler: StreamErrorHandler,
{
    error_handler: TErrorHandler,
}

impl<T: StreamErrorHandler> StreamErrorHandlerWrapper<T> {
    fn new(output: T) -> Self {
        StreamErrorHandlerWrapper {
            error_handler: output,
        }
    }
}

impl<T: StreamErrorHandler> UnsafeSCStreamOutput for StreamErrorHandlerWrapper<T> {
    fn got_sample(&self) {
        self.error_handler.on_error();
    }
}

#[derive(Debug)]
pub struct StreamOutputWrapper<T: StreamOutput> {
    output: T,
}

impl<T: StreamOutput> StreamOutputWrapper<T> {
    fn new(output: T) -> Self {
        StreamOutputWrapper { output }
    }
}

impl<TOutput> UnsafeSCStreamOutput for StreamOutputWrapper<TOutput>
where
    TOutput: StreamOutput,
{
    fn got_sample(&self) {
        self.output.stream_output();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct SomeOutputWrapper {}
    impl StreamOutput for SomeOutputWrapper {
        fn stream_output(&self) {}
    }
    #[test]
    fn test_output_wrapper() {
        let output_wrapper = StreamOutputWrapper::new(SomeOutputWrapper {});
        // let tream = SCStream::new();
    }
}
