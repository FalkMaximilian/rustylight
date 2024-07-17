use anyhow::Result;
use v4l::buffer::Type;
use v4l::io::mmap::Stream;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::Device;
use v4l::Format;
use v4l::FourCC;

use crate::settings::Settings;

pub struct Video {}

impl Video {
    /// Create a new Stream with the given settings
    ///
    /// Settings.processing_resolution will be used to set the resolution at which the frames will be
    /// processed.
    pub fn new(settings: &Settings) -> Result<(Stream, Format)> {
        let mut input = Device::new(settings.video_device)?;

        // Let's say we want to explicitly request another format
        let mut fmt = dev.format()?;
        let (width, height): (f64, f64) = settings.processing_resolution.into();
        fmt.width = width;
        fmt.height = height;
        fmt.fourcc = FourCC::new(b"YUYV");
        let fmt = dev.set_format(&fmt)?;

        let mut stream = Stream::with_buffers(&mut dev, Type::VideoCapture, 4)?;

        Ok((stream, fmt))
    }
}

/// Convert frame from YUYV to RGB3
///
/// TODO: SIMD for better performance
pub fn process_frame(frame: &[u8], rgb_target: &mut Vec<(u8, u8, u8)>) {
    //println!("Source frame has length: {}", frame.len());
    for (i, chunk) in frame.chunks_exact(4).enumerate() {
        let y0 = (chunk[0] - 16) as f32;
        let u = (chunk[1] - 128) as f32;
        let y1 = (chunk[2] - 16) as f32;
        let v = (chunk[3] - 128) as f32;

        let r0 = (y0 + 1.402 * v).clamp(0.0, 255.0) as u8;
        let g0 = (y0 - 0.344136 * u - 0.714136 * v).clamp(0.0, 255.0) as u8;
        let b0 = (y0 + 1.772 * u).clamp(0.0, 255.0) as u8;

        let r1 = (y1 + 1.402 * v).clamp(0.0, 255.0) as u8;
        let g1 = (y1 - 0.344136 * u - 0.714136 * v).clamp(0.0, 255.0) as u8;
        let b1 = (y1 + 1.772 * u).clamp(0.0, 255.0) as u8;

        rgb_target[i * 2] = (r0, g0, b0);
        rgb_target[(i * 2) + 1] = (r1, g1, b1);
    }
}
