// use gst_video::VideoFormat;
// use screencapturekit::{cm_sample_buffer::CMSampleBuffer, sc_types::four_char_code::FourCharCode};



//
// fn apple_format_to_gst_format(raw_code: u32) -> VideoFormat {
//     let str = FourCharCode::from_int(raw_code).to_string();
//     match str.as_str() {
//         "BGRA" => VideoFormat::Bgra,
//         "l10r" => VideoFormat::Gbra10le,
//         "420v" => VideoFormat::Nv12,
//         "420f" => VideoFormat::Nv12,
//         _ => VideoFormat::Unknown,
//     }
// }

// fn core_media_buffer_new(sample_buf: CMSampleBuffer) -> gst::Buffer {
//     todo!();
//     // let format =
//     // let video_info = gst_video::VideoInfo::builder(format, width, height).build();
// }
//
//
//   GstBuddffer *buf;
//   CVImageBufferRef image_buf = CMSampleBufferGetImageBuffer(sample_buf);
//
//   buf = gst_buffer_new();
//
//   gst_core_media_meta_add(buf, image_buf);
//   if (image_buf != NULL && CFGetTypeID(image_buf) == CVPixelBufferGetTypeID()) {
//     GstVideoInfo info;
//     CVPixelBufferRef pixel_buf = (CVPixelBufferRef)image_buf;
//     if (!gst_video_info_init_from_pixel_buffer(&info, pixel_buf)) {
//       goto error;
//     }
//     gst_core_video_wrap_pixel_buffer(buf, &info, pixel_buf);
//   } else {
//     goto error;
//   }
//
//   return buf;
//
// error:
//   if (buf) {
//     gst_buffer_unref(buf);
//   }
//
//   return NULL;
// }
