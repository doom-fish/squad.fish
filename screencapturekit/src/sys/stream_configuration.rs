use objc::{msg_send, *, runtime::Class};

use objc_foundation::INSObject;
use objc_id::Id;

use crate::shared::{OSType, Rect};

pub struct UnsafeSCStreamConfiguration;
unsafe impl Message for UnsafeSCStreamConfiguration {}
impl From<SCStreamConfiguration> for Id<UnsafeSCStreamConfiguration> {
    fn from(value: SCStreamConfiguration) -> Self {
        let unsafe_ref = UnsafeSCStreamConfiguration::new();
        unsafe {
             let _:() = msg_send![unsafe_ref, setWidth: value.width];
             let _:() = msg_send![unsafe_ref, setHeight: value.height];
        }
        unsafe_ref
    }
}

impl INSObject for UnsafeSCStreamConfiguration {
    fn class() -> &'static Class {
        Class::get("SCStreamConfiguration")
                .expect("Missing SCStreamConfiguration class, check that the binary is linked with ScreenCaptureKit")
    }
}

#[derive(Default)]
pub struct SCStreamConfiguration {
    // The width of the output.
    width: u32,
    //   The height of the output.
    height: u32,
    // A boolean value that indicates whether to scale the output to fit the configured width and height.
    scales_to_fit: bool,
    // A rectangle that specifies the source area to capture.
    source_rect: Rect,
    // A rectangle that specifies a destination into which to write the output.
    destination_rect: Rect,
    // Configuring Colors
    // A pixel format for sample buffers that a stream outputs.
    pixel_format: OSType,
    // A color matrix to apply to the output surface.
    color_matrix: String,
    // A color space to use for the output buffer.
    color_space_name: String,
    // A background color for the output.
    // Controlling Visibility
    // Todo: Implement Color struct
    //backgroundColor: Color,
    // A boolean value that determines whether the cursor is visible in the stream.
    shows_cursor: bool,
    // Optimizing Performance
    // The maximum number of frames for the queue to store.
    queue_depth: u32,
    // The desired minimum time between frame updates, in seconds.
    minimum_frameinterval: u64,
    // Configuring Audio
    // A boolean value that indicates whether to capture audio.
    captures_audio: bool,
    // The sample rate for audio capture.
    sample_rate: u32,
    // The number of audio channels to capture.
    channel_count: u32,
    // A boolean value that indicates whether to exclude a
    excludes_current_process_audio: bool,
}

#[cfg(test)]
mod get_shareable_content {

    use super::*;
    #[test]
    fn test_from() {
        let _: Id<UnsafeSCStreamConfiguration> = SCStreamConfiguration::default().into();
    }
}
