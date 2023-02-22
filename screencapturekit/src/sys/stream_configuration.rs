use objc::{msg_send, runtime::Class, *};

use objc_foundation::INSObject;
use objc_id::Id;

use crate::os_types::{
    base::{CMTime, OSType, UInt32, BOOL},
    geometry::CGRect,
    graphics::CGColor,
};

pub struct UnsafeStreamConfiguration;
unsafe impl Message for UnsafeStreamConfiguration {}
impl From<SCStreamConfiguration> for Id<UnsafeStreamConfiguration> {
    fn from(value: SCStreamConfiguration) -> Self {
        let unsafe_ref = UnsafeStreamConfiguration::new();
        unsafe {
            let _: () = msg_send![unsafe_ref, setWidth: value.width];
            let _: () = msg_send![unsafe_ref, setHeight: value.height];
            let _: () = msg_send![unsafe_ref, scaleToFit: value.scales_to_fit];
            let _: () = msg_send![unsafe_ref, sourceRect: value.source_rect];
            let _: () = msg_send![unsafe_ref, destinationRect: value.destination_rect];
            let _: () = msg_send![unsafe_ref, pixelFormat: value.pixel_format];
            let _: () = msg_send![unsafe_ref, colorMatrix: value.color_matrix];
            let _: () = msg_send![unsafe_ref, colorSpaceName: value.color_space_name];
            let _: () = msg_send![unsafe_ref, backgroundColor: value.background_color];
            let _: () = msg_send![unsafe_ref, showCursor: value.shows_cursor];
            let _: () = msg_send![unsafe_ref, queueDepth: value.queue_depth];
            let _: () = msg_send![unsafe_ref, minimumFrameInterval: value.minimum_frame_interval];
        }
        unsafe_ref
    }
}

impl INSObject for UnsafeStreamConfiguration {
    fn class() -> &'static Class {
        Class::get("SCStreamConfiguration")
                .expect("Missing SCStreamConfiguration class, check that the binary is linked with ScreenCaptureKit")
    }
}

#[derive(Default, Debug)]
pub struct SCStreamConfiguration {
    // The width of the output.
    width: UInt32,
    //   The height of the output.
    height: UInt32,
    // A boolean value that indicates whether to scale the output to fit the configured width and height.
    scales_to_fit: BOOL,
    // A rectangle that specifies the source area to capture.
    source_rect: CGRect,
    // A rectangle that specifies a destination into which to write the output.
    destination_rect: CGRect,
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
    background_color: CGColor,

    // A boolean value that determines whether the cursor is visible in the stream.
    shows_cursor: BOOL,
    // Optimizing Performance
    // The maximum number of frames for the queue to store.
    queue_depth: UInt32,
    // The desired minimum time between frame updates, in seconds.
    minimum_frame_interval: CMTime,
    // Configuring Audio
    // A boolean value that indicates whether to capture audio.
    captures_audio: BOOL,
    // The sample rate for audio capture.
    sample_rate: UInt32,
    // The number of audio channels to capture.
    channel_count: UInt32,
    // A boolean value that indicates whether to exclude a
    excludes_current_process_audio: BOOL,
}

#[cfg(test)]
mod get_shareable_content {

    use super::*;
    #[test]
    fn test_from() {
        let _: Id<UnsafeStreamConfiguration> = SCStreamConfiguration::default().into();
    }
}
