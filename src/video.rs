use anyhow::Result;
use opencv::videoio::{
    VideoCapture, VideoCaptureTrait, CAP_ANY, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH,
};

use crate::settings::Settings;

pub struct Video {}

impl Video {
    pub fn new(settings: &Settings) -> Result<VideoCapture> {
        let mut input = VideoCapture::new(settings.video_device, CAP_ANY)?;
        Video::set_processing_resolution(&mut input, settings.processing_resolution.into());
        Ok(input)
    }

    fn set_processing_resolution(device: &mut VideoCapture, resolution: (f64, f64)) {
        let _ = device.set(CAP_PROP_FRAME_WIDTH, resolution.0);
        let _ = device.set(CAP_PROP_FRAME_HEIGHT, resolution.1);
    }
}
