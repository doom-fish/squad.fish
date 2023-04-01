use std::marker::PhantomData;

use screencapturekit_sys::{
    os_types::rc::Id,
    stream::{UnsafeSCStream, UnsafeSCStreamError, UnsafeSCStreamHandle, UnsafeSCStreamOutput},
};

use crate::{sc_content_filter::SCContentFilter, sc_stream_configuration::SCStreamConfiguration};

#[derive(Debug)]
pub struct SCStream {
    pub(crate) _unsafe_ref: Id<UnsafeSCStream>,
}

#[derive(Clone, Copy)]
struct SCStreamHandle<F>
where
    F: Fn(),
{
    output: F,
}

impl<F> SCStreamHandle<F>
where
    F: Fn(),
{
    fn new(output: F) -> Self {
        SCStreamHandle { output }
    }
}

impl<F> UnsafeSCStreamError for SCStreamHandle<F> where F: Fn() {}
impl<F> UnsafeSCStreamOutput for SCStreamHandle<F>
where
    F: Fn(),
{
    fn got_sample(&self) {
        (self.output)()
    }
}

#[test]
fn name() {
    let a = Some(SCStreamHandle::new(|| {
        println!("HEJ!");
    }));

    UnsafeSCStreamHandle::init(a, a);
}

// impl SCStream {
//     fn new(filter: SCContentFilter, config: SCStreamConfiguration) -> Self {
//        let unsafe_sc_stream = UnsafeSCStream::new();
//
//         SCStream {
//             _unsafe_ref: unsafe_sc_stream,
//         }
//     }
// }
