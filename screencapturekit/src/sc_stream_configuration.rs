use screencapturekit_sys::{
    os_types::{four_char_code::FourCharCode, rc::Id},
    stream_configuration::{UnsafeStreamConfiguration, UnsafeStreamConfigurationRef},
};

pub static PixelFormats: [PixelFormat; 4] = [
    PixelFormat::ARGB8888,
    PixelFormat::ARGB2101010,
    PixelFormat::YCbCr420f,
    PixelFormat::YCbCr420v,
];

#[derive(Default, Debug)]
pub struct OutputSizeConfig {
    // The width of the output.
    pub width: u32,
    //   The height of the output.
    pub height: u32,
    // A boolean value that indicates whether to scale the output to fit the configured width and height.
    pub scales_to_fit: bool,
    // A background color for the output.
    // Controlling Visibility
    // pub background_color: Color,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum PixelFormat {
    ARGB8888,
    ARGB2101010,
    YCbCr420v,
    #[default]
    YCbCr420f,
}

impl From<PixelFormat> for FourCharCode {
    fn from(val: PixelFormat) -> Self {
        match val {
            PixelFormat::ARGB8888 => FourCharCode::from_chars(*b"BGRA"),
            PixelFormat::ARGB2101010 => FourCharCode::from_chars(*b"l10r"),
            PixelFormat::YCbCr420v => FourCharCode::from_chars(*b"420v"),
            PixelFormat::YCbCr420f => FourCharCode::from_chars(*b"420f"),
        }
    }
}

#[derive(Default, Debug)]
pub struct OutputCodingConfig {
    // Configuring Colors
    // A pixel format for sample buffers that a stream outputs.
    pub pixel_format: PixelFormat,
    // A color matrix to apply to the output surface.
    pub color_matrix: String,
    // A color space to use for the output buffer.
    pub color_space_name: String,
    // A background color for the output.
    // Controlling Visibility
    // Todo: Implement Color struct
    // pub background_color: Color,
}

#[derive(Default, Debug)]
pub struct CaptureConfig {
    // A boolean value that determines whether the cursor is visible in the stream.
    pub shows_cursor: bool,
    // Optimizing Performance
    // The maximum number of frames for the queue to store.
    pub queue_depth: u32,
    // The desired minimum time between frame updates, in seconds.
    pub minimum_frame_interval: u64,
    // Configuring Audio
    // A boolean value that indicates whether to capture audio.
    pub captures_audio: bool,
    // The sample rate for audio capture.
    pub sample_rate: u32,
    // The number of audio channels to capture.
    pub channel_count: u32,
    // A boolean value that indicates whether to exclude a
    pub excludes_current_process_audio: bool,
}

pub enum ConfigParams {
    Full(OutputSizeConfig, OutputCodingConfig, CaptureConfig),
    Size { width: u32, height: u32 },
}

#[derive(Debug)]
pub struct SCStreamConfiguration {
    pub(crate) _unsafe_ref: Id<UnsafeStreamConfigurationRef>,
}
impl SCStreamConfiguration {
    pub fn new(params: ConfigParams) -> Self {
        let unsafe_config = match params {
            ConfigParams::Full(size, _coding, _capture) => UnsafeStreamConfiguration {
                width: size.width,
                height: size.height,
                ..Default::default()
            },
            ConfigParams::Size { width, height } => UnsafeStreamConfiguration {
                width,
                height,
                ..Default::default()
            },
        };
        SCStreamConfiguration {
            _unsafe_ref: unsafe_config.into(),
        }
    }
}

#[cfg(test)]
mod get_configuration {

    use super::*;
    #[test]
    fn test_configuration() {
        SCStreamConfiguration::new(ConfigParams::Size {
            width: 100,
            height: 100,
        });
    }
}
