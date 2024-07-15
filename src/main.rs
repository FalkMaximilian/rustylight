#![allow(dead_code)]
#![allow(unreachable_code)]

mod cli;
mod translation_engine;

use std::{env, os::unix::thread, time::Duration};

use anyhow::Result;
use opencv::{
    core::{flip, transpose, Rect, Scalar, Vec3b, CV_8UC3},
    highgui, imgproc,
    prelude::*,
    videoio::{self, VideoCapture, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH},
};
use smart_leds::RGB8;

use cli::{Direction, RustylightCli, StartCorner};
use translation_engine::TranslationEngine;

use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use std::thread::sleep;

fn mat_to_rgb8_array(mat: &Mat) -> Result<Vec<RGB8>> {
    // Ensure the Mat is of the correct type
    if mat.typ() != CV_8UC3 {
        println!("Mat in incorrect format!");
    }

    let rows = mat.rows();
    let cols = mat.cols();
    let mut rgb8_array = Vec::with_capacity((rows * cols) as usize);

    for row in 0..rows {
        for col in 0..cols {
            let pixel = mat.at_2d::<Vec3b>(row, col)?;
            rgb8_array.push(RGB8 {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            })
        }
    }
    Ok(rgb8_array)
}

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

    let log_level: Level = match env::var("LOG_LEVEL").as_deref() {
        Ok("INFO") => Level::INFO,
        Ok("DEBUG") => Level::DEBUG,
        Ok("TRACE") => Level::TRACE,
        _ => Level::TRACE,
    };

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(log_level)
        // completes the builder.
        .finish();

    // Steup for tracing
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Cli
    let cli = RustylightCli::setup();

    let border_thickness = env::var("BORDER_THICKNESS")?.parse()?;

    #[cfg(feature = "highgui")]
    {
        highgui::named_window("original", highgui::WINDOW_NORMAL)?;
        highgui::named_window("frame", highgui::WINDOW_NORMAL)?;
    }

    let mut orig_frame = Mat::default();

    let mut cam = videoio::VideoCapture::new(4, videoio::CAP_ANY)?;
    //let mut cam =
    //    videoio::VideoCapture::from_file("/home/max/Downloads/test_vid_02.mp4", videoio::CAP_ANY)?;

    // Video input usualy does not need to be
    cam.set(CAP_PROP_FRAME_WIDTH, 1280.0)?;
    cam.set(CAP_PROP_FRAME_HEIGHT, 720.0)?;

    // Get the size of the video feed
    wait_for_frame(&mut cam, &mut orig_frame);
    let size = orig_frame.size()?;
    info!(
        "Reading video data with resolution widht: {}, height: {}",
        size.width, size.height
    );

    // The border must be smaller than half of width and height
    if size.width / 2 < border_thickness || size.height < border_thickness {
        info!(
            "Border is too thick! The following must hold: border < width/2 && border < height/2"
        );
        return Ok(());
    }

    // Set the width of "regions"
    let region_width = size.width - border_thickness;
    let region_height = size.height - border_thickness;

    // Create the target target_frame
    // This will hold the data that shall be sent to the leds
    let mut target_frame = Mat::new_rows_cols_with_default(
        40,
        (2 * region_height) + (2 * region_width),
        orig_frame.typ(),
        Scalar::all(0.0),
    )?;

    // Translation funcs that shall be applied to each frame
    let translation_funcs = TranslationEngine::new(
        cli.start_corner,
        cli.direction,
        region_width,
        region_height,
        border_thickness,
    );

    info!("----- STARTING MAIN LOOP -----");
    loop {
        wait_for_frame(&mut cam, &mut orig_frame);

        for func in translation_funcs.iter() {
            func(&orig_frame, &mut target_frame)?;
        }

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
