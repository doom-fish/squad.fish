use screencapturekit_sys::stream_output_handler::UnsafeSCStreamOutput;

pub trait StreamOutput {
    fn stream_output(&self);
}

pub(crate) struct StreamOutputWrapper<'a, T: StreamOutput>(&'a T);

impl<'a, T: StreamOutput> StreamOutputWrapper<'a, T> {
    pub fn new(output: &'a T) -> Self {
        StreamOutputWrapper(output)
    }
}

impl<'a, TOutput> UnsafeSCStreamOutput for StreamOutputWrapper<'a, TOutput>
where
    TOutput: StreamOutput,
{
    fn got_sample(&self) {
        self.0.stream_output();
    }
}
