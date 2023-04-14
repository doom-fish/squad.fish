use screencapturekit_sys::stream_output_handler::UnsafeSCStreamOutput;

pub trait StreamOutput: Sync + Send + 'static {
    fn stream_output(&self);
}

pub(crate) struct StreamOutputWrapper<T: StreamOutput>(T);

impl<T: StreamOutput> StreamOutputWrapper<T> {
    pub fn new(output: T) -> Self {
        Self(output)
    }
}

struct SampleBuffer {

}

impl<TOutput: StreamOutput> UnsafeSCStreamOutput for StreamOutputWrapper<TOutput> {
    fn got_sample(&self, ) {
        self.0.stream_output();
    }
}
