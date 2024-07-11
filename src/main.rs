use std::env;

use anyhow::{Result};
use opencv::{self as cv, core::{flip, transpose, Rect, Scalar, CV_8UC3, Vec3b}, highgui, imgproc, prelude::*, videoio};
use smart_leds::RGB8;

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

fn main() -> Result<()> {
    dotenvy::dotenv()?;

    #[cfg(feature = "highgui")]
    {
        highgui::named_window("original", highgui::WINDOW_NORMAL)?;
        highgui::named_window("frame", highgui::WINDOW_NORMAL)?;
    }

    //let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut cam = videoio::VideoCapture::from_file("/home/max/Downloads/test_vid.mp4", videoio::CAP_ANY)?;
    let mut orig_frame = Mat::default();

    // Get the size of the video feed
    cam.read(&mut orig_frame)?;
    let size = orig_frame.size()?;

    let border_thickness = env::var("BORDER_THICKNESS")?.parse()?;

    let horizontal_main = orig_frame.cols();
    let vertical_main = orig_frame.rows() - (2* border_thickness);

    let mut border_frame = Mat::new_rows_cols_with_default(border_thickness, (2*orig_frame.cols())+(2*(orig_frame.rows() - 2*border_thickness)), orig_frame.typ(), Scalar::all(0.0))?;

    // Borders of the video 
    let borders = [
        Rect::new(0, 0, size.width, border_thickness),
        Rect::new(0, size.height - border_thickness, size.width, border_thickness),
        Rect::new(0, border_thickness, border_thickness, size.height-(2*border_thickness)),
        Rect::new(size.width - border_thickness, border_thickness, border_thickness, size.height - (2*border_thickness)),
    ];

    loop {
        cam.read(&mut orig_frame)?;

        // Copy top border to destionation
        let src_roi = Mat::roi(&orig_frame, borders[0])?;
        let mut dst_roi = Mat::roi_mut(&mut border_frame, borders[0])?;
        src_roi.copy_to(&mut dst_roi)?;


        // Transpose and flip right border
        let src_roi = Mat::roi(&orig_frame, borders[3])?;
        let mut right_transposed_roi = Mat::default();
        transpose(&src_roi, &mut right_transposed_roi)?;
        let mut right_flipped_roi = Mat::default();
        flip(&right_transposed_roi, &mut right_flipped_roi, 0)?;

        // Copy to destination
        let right_border_target = Rect::new(horizontal_main, 0, vertical_main, border_thickness);
        let mut dst_roi = Mat::roi_mut(&mut border_frame, right_border_target)?;
        right_flipped_roi.copy_to(&mut dst_roi)?;


        // Flip bottom roi 
        let src_roi = Mat::roi(&orig_frame, borders[1])?;
        let mut bottom_flipped_roi = Mat::default();
        flip(&src_roi, &mut bottom_flipped_roi, -1)?;

        // Copy flipped
        let bottom_border_target = Rect::new(horizontal_main + vertical_main, 0, horizontal_main, border_thickness);
        let mut dst_roi = Mat::roi_mut(&mut border_frame, bottom_border_target)?;
        bottom_flipped_roi.copy_to(&mut dst_roi)?;


        // Transpose left border
        let src_roi = Mat::roi(&orig_frame, borders[2])?;
        let mut left_transposed_roi = Mat::default();
        transpose(&src_roi, &mut left_transposed_roi)?;
        let mut left_flipped = Mat::default();
        flip(&left_transposed_roi, &mut left_flipped, 1)?;

        // Copy to destination
        let left_border_target = Rect::new(2*horizontal_main + vertical_main, 0, vertical_main, border_thickness);
        let mut dst_roi = Mat::roi_mut(&mut border_frame, left_border_target)?;
        left_flipped.copy_to(&mut dst_roi)?;

        // To RGB8 for driving LEDs
        println!("{:?}", mat_to_rgb8_array(&border_frame));

        #[cfg(feature = "highgui")]
        {
            highgui::imshow("original", &orig_frame)?;
            highgui::imshow("frame", &border_frame)?;

            let key = highgui::wait_key(1)?;
            if key == 113 { // quit with q
                break;
            }
        }
    }

    Ok(())
}