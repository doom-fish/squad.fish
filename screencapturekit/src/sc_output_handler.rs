use screencapturekit_sys::stream_output_handler::UnsafeSCStreamOutput;

pub trait StreamOutput {
    fn stream_output(&self);
}

pub(crate) struct StreamOutputWrapper<T: StreamOutput>(T);

impl<T: StreamOutput> StreamOutputWrapper<T> {
    //pub fn new(output: T) -> Self {
    //     StreamOutputWrapper(output)
    // }
}

impl<TOutput> UnsafeSCStreamOutput for StreamOutputWrapper<TOutput>
where
    TOutput: StreamOutput,
{
    fn got_sample(&self) {
        self.0.stream_output();
    }
}
