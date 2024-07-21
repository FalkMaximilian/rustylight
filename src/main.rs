mod lightstrip;
mod settings;
mod translation_engine;
mod video;

use lightstrip::Lightstrip;
use std::time::Duration;
use v4l::io::traits::CaptureStream;
use v4l::Format;
use v4l::Stream;
use video::process_frame;

use anyhow::Result;
use settings::Settings;

use translation_engine::TranslationEngine;

use tracing::{debug, error, info};
use tracing_subscriber::FmtSubscriber;
use video::Video;

use std::thread::sleep;

fn main() -> Result<()> {
    let settings = Settings::new()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(settings.log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!(
        "Rustylight will use the following settings: {:?}",
        &settings
    );

    let (mut stream, fmt) = Video::new(&settings)?;
    info!("Format in use:\n{}", fmt);

    let (buf, _meta) = stream.next().unwrap();

    // The border must be smaller than half of width and height
    if fmt.width / 2 < settings.capture_area_size || fmt.height < settings.capture_area_size {
        info!(
            "Border is too thick! The following must hold: border < width/2 && border < height/2"
        );
        return Ok(());
    }

    // Set the width of "regions"
    let region_width = fmt.width - settings.capture_area_size;
    let region_height = fmt.height - settings.capture_area_size;

    let pixel_per_led = ((2 * region_height) + (2 * region_width)) / settings.led_count;
    info!("Pixels per LED: {}", pixel_per_led);

    let mut temp_vec: Vec<(u32, u32, u32)> =
        Vec::with_capacity(((2 * region_height) + (2 * region_height)) * 3 as usize);

    // let mut led_values: Vec<(u32, u32, u32)> = Vec::with_capacity(settings.led_count as usize);
    let mut ledstrip = Lightstrip::new(settings.led_count);

    // Translation funcs that shall be applied to each frame
    let translation_funcs = TranslationEngine::new(
        settings.start_corner,
        settings.direction,
        region_width,
        region_height,
        settings.capture_area_size,
        fmt.width,
        fmt.height,
        pixel_per_led,
    );

    info!("----- STARTING MAIN LOOP -----");
    loop {
        let (buf, _meta) = stream.next().unwrap();
        process_frame(buf, temp_vec);

        for func in translation_funcs.iter() {
            func(&temp_vec, &ledstrip);
        }
    }

    Ok(())
}
