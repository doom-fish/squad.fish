use screencapturekit_sys::{os_types::rc::Id, stream::UnsafeSCStream};

use crate::{sc_content_filter::SCContentFilter, sc_stream_configuration::SCStreamConfiguration};

#[derive(Debug)]
pub struct SCStream {
    pub(crate) _unsafe_ref: Id<UnsafeSCStream>,
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
