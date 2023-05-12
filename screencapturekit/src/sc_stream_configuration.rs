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
#[derive(Default, Debug, Clone, Copy)]
pub struct SizeConfig {
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

#[derive(Default, Clone, Debug, Copy)]
pub struct OutputCodingConfig {
    // Configuring Colors
    // A pixel format for sample buffers that a stream outputs.
    pub pixel_format: PixelFormat,
    // A color matrix to apply to the output surface.
    pub color_matrix: &'static str,
    // A color space to use for the output buffer.
    pub color_space_name: &'static str,
    // A background color for the output.
    // Controlling Visibility
    // Todo: Implement Color struct
    // pub background_color: Color,
}

#[derive(Default, Clone, Debug, Copy)]
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
    Full(SizeConfig, OutputCodingConfig, CaptureConfig),
    Size(SizeConfig),
}

#[derive(Debug)]
pub struct SCStreamConfiguration {
    pub size: SizeConfig,
    pub capture: CaptureConfig,
    pub output: OutputCodingConfig,
    pub(crate) _unsafe_ref: Option<Id<UnsafeStreamConfigurationRef>>,
}
impl Default for SCStreamConfiguration {
    fn default() -> Self {
        Self {
            size: Default::default(),
            capture: Default::default(),
            output: Default::default(),
            _unsafe_ref: None,
        }
    }
}
impl SCStreamConfiguration {
    pub fn new(params: ConfigParams) -> Self {
        match params {
            ConfigParams::Full(size, output, capture) => SCStreamConfiguration {
                size,
                output,
                capture,
                _unsafe_ref: UnsafeStreamConfiguration {
                    width: size.width,
                    height: size.height,
                    ..Default::default()
                }
                .into(),
            },
            ConfigParams::Size(size) => SCStreamConfiguration {
                size,
                _unsafe_ref: UnsafeStreamConfiguration {
                    width: size.width,
                    height: size.height,
                    scales_to_fit: size.scales_to_fit as i8,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod get_configuration {

    use super::*;
    #[test]
    fn test_configuration() {
        SCStreamConfiguration::new(ConfigParams::Size(SizeConfig {
            width: 100,
            height: 100,
            scales_to_fit: false,
        }));
    }
}
