#![allow(dead_code)]
#![allow(unreachable_code)]

mod settings;
mod translation_engine;
mod video;

use std::time::Duration;

use anyhow::Result;
use opencv::{
    core::{Scalar, Vec3b, CV_8UC3},
    highgui,
    prelude::*,
    videoio::VideoCapture,
};
use settings::Settings;

use translation_engine::TranslationEngine;

use tracing::{debug, error, info};
use tracing_subscriber::FmtSubscriber;
use video::Video;

use std::thread::sleep;

use smart_leds::{SmartLedsWrite, RGB8};
use ws281x_rpi::Ws2812Rpi;

fn vec3b_to_smaller_rgb8(temp: &Vec<Vec3b>, pixel_per_led: i32) -> Vec<RGB8> {
    let mut pixels: Vec<RGB8> = Vec::new();

    for chunk in temp.chunks(pixel_per_led as usize) {
        let mut mean_b = 0;
        let mut mean_g = 0;
        let mut mean_r = 0;

        for elem in chunk.iter() {
            mean_b += elem[0] as u32;
            mean_g += elem[1] as u32;
            mean_r += elem[2] as u32;
        }

        mean_b /= pixel_per_led as u32;
        mean_g /= pixel_per_led as u32;
        mean_r /= pixel_per_led as u32;

        pixels.push(RGB8 {
            r: mean_r as u8,
            g: mean_g as u8,
            b: mean_b as u8,
        })
    }

    pixels
}

/// If a size-zero has been received wait for half a second and try again
fn wait_for_frame(v: &mut VideoCapture, f: &mut Mat) {
    loop {
        v.read(f).expect("Could not read frame.");
        let size = f.size().expect("Could not get size from frame");
        if size.width == 0 || size.height == 0 {
            debug!("Input with invalid size. Waiting...");
            sleep(Duration::from_millis(500));
            continue;
        }
        break;
    }
}

fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let settings = Settings::new()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(settings.log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!(
        "Rustylight will use the following settings: {:?}",
        &settings
    );

    #[cfg(feature = "highgui")]
    {
        highgui::named_window("original", highgui::WINDOW_NORMAL)?;
        highgui::named_window("frame", highgui::WINDOW_NORMAL)?;
    }

    let mut orig_frame = Mat::default();

    let mut input = Video::new(&settings)?;
    //let mut cam =
    //    videoio::VideoCapture::from_file("/home/max/Downloads/test_vid_02.mp4", videoio::CAP_ANY)?;

    // Get the size of the video feed
    wait_for_frame(&mut input, &mut orig_frame);
    let size = orig_frame.size()?;
    info!(
        "Reading video data with resolution widht: {}, height: {}",
        size.width, size.height
    );

    // The border must be smaller than half of width and height
    if size.width / 2 < settings.capture_area_size || size.height < settings.capture_area_size {
        info!(
            "Border is too thick! The following must hold: border < width/2 && border < height/2"
        );
        return Ok(());
    }

    // Set the width of "regions"
    let region_width = size.width - settings.capture_area_size;
    let region_height = size.height - settings.capture_area_size;

    let pixel_per_led = ((2 * region_height) + (2 * region_width)) / settings.led_count;
    info!("Pixels per LED: {}", pixel_per_led);

    // Create the target target_frame
    // This will hold the data that shall be sent to the leds
    //let mut target_frame = Mat::new_rows_cols_with_default(
    //    1,
    //    (2 * region_height) + (2 * region_width),
    //    orig_frame.typ(),
    //    Scalar::all(0.0),
    //)?;

    let mut target_vec: Vec<Vec3b> =
        Vec::with_capacity(((2 * region_height) + (2 * region_height)) as usize);

    let mut led_values: Vec<RGB8> = Vec::with_capacity(settings.led_count as usize);
    let mut ws = Ws2812Rpi::new(settings.led_count, 18)?;

    // Translation funcs that shall be applied to each frame
    let translation_funcs = TranslationEngine::new(
        settings.start_corner,
        settings.direction,
        region_width,
        region_height,
        settings.capture_area_size,
    );

    info!("----- STARTING MAIN LOOP -----");
    loop {
        wait_for_frame(&mut input, &mut orig_frame);

        for func in translation_funcs.iter() {
            func(&orig_frame, &mut target_vec)?;
        }

        ws.write(
            vec3b_to_smaller_rgb8(&target_vec, pixel_per_led)
                .iter()
                .cloned(),
        )
        .unwrap();

        #[cfg(feature = "highgui")]
        {
            highgui::imshow("original", &orig_frame)?;
            highgui::imshow("frame", &target_frame)?;

            let key = highgui::wait_key(1)?;
            if key == 113 {
                // quit with q
                break;
            }
        }
    }

    Ok(())
}
