use anyhow::Result;
use opencv::core::{Mat, MatTrait, MatTraitConst, Rect, Vec3b};

use crate::cli::{Direction, StartCorner};

use tracing::debug;

// Roi, Target Mat, Offset
type Action = Box<dyn Fn(&Mat, &mut Mat) -> Result<()>>;

#[derive(Debug)]
enum EdgeDirection {
    RTL,
    LTR,
    TTB,
    BTT,
}

pub struct TranslationEngine {}

impl TranslationEngine {
    pub fn new(
        start: StartCorner,
        direction: Direction,
        width: i32,
        height: i32,
        thickness: i32,
    ) -> [Action; 4] {
        debug!(
            "Setting up frame translation for start: {:?} direction: {:?} border_thickness: {}",
            start, direction, thickness
        );
        match direction {
            Direction::CW => Self::get_translation_funcs_cw(start, width, height, thickness),
            Direction::CCW => Self::get_translation_funcs_ccw(start, width, height, thickness),
        }
    }

    // Creates an array of exactly four functions that later shall be applied on an input frame.
    // There are four possible cases depending on which corner the led_strip starts at. This gives
    // the functions for clockwise.
    fn get_translation_funcs_cw(
        start: StartCorner,
        width: i32,
        height: i32,
        thickness: i32,
    ) -> [Action; 4] {
        debug!(
            "Setting up translation functions for clockwise layout starting from {:?}",
            start
        );
        let top_region = Rect::new(0, 0, width, thickness);
        let right_region = Rect::new(width, 0, thickness, height);
        let bottom_region = Rect::new(thickness, height, width, thickness);
        let left_region = Rect::new(0, thickness, thickness, height);

        match start {
            StartCorner::TL => [
                Self::translation_func(EdgeDirection::LTR, 0, top_region),
                Self::translation_func(EdgeDirection::TTB, width, right_region),
                Self::translation_func(EdgeDirection::RTL, width + height, bottom_region),
                Self::translation_func(EdgeDirection::BTT, width + height + width, left_region),
            ],
            StartCorner::TR => [
                Self::translation_func(EdgeDirection::TTB, 0, right_region),
                Self::translation_func(EdgeDirection::RTL, height, bottom_region),
                Self::translation_func(EdgeDirection::BTT, height + width, left_region),
                Self::translation_func(EdgeDirection::LTR, height + width + height, top_region),
            ],
            StartCorner::BR => [
                Self::translation_func(EdgeDirection::RTL, 0, bottom_region),
                Self::translation_func(EdgeDirection::BTT, width, left_region),
                Self::translation_func(EdgeDirection::LTR, width + height, top_region),
                Self::translation_func(EdgeDirection::TTB, width + height + width, right_region),
            ],
            StartCorner::BL => [
                Self::translation_func(EdgeDirection::BTT, 0, left_region),
                Self::translation_func(EdgeDirection::LTR, height, top_region),
                Self::translation_func(EdgeDirection::TTB, height + width, right_region),
                Self::translation_func(EdgeDirection::RTL, height + width + height, bottom_region),
            ],
        }
    }

    // Creates an array of exactly four functions that later shall be applied on an input frame.
    // There are four possible cases depending on which corner the led_strip starts at. This gives
    // the functions for counter clockwise.
    fn get_translation_funcs_ccw(
        start: StartCorner,
        width: i32,
        height: i32,
        thickness: i32,
    ) -> [Action; 4] {
        debug!(
            "Setting up translation functions for counter clockwise layout starting from {:?}",
            start
        );
        let top_region = Rect::new(thickness, 0, width, thickness);
        let right_region = Rect::new(width, thickness, thickness, height);
        let bottom_region = Rect::new(0, height, width, thickness);
        let left_region = Rect::new(0, 0, thickness, height);

        match start {
            StartCorner::TL => [
                Self::translation_func(EdgeDirection::TTB, 0, left_region),
                Self::translation_func(EdgeDirection::LTR, height, bottom_region),
                Self::translation_func(EdgeDirection::BTT, height + width, right_region),
                Self::translation_func(EdgeDirection::RTL, height + width + height, top_region),
            ],
            StartCorner::BL => [
                Self::translation_func(EdgeDirection::LTR, 0, bottom_region),
                Self::translation_func(EdgeDirection::BTT, width, right_region),
                Self::translation_func(EdgeDirection::RTL, width + height, top_region),
                Self::translation_func(EdgeDirection::TTB, width + height + width, left_region),
            ],
            StartCorner::BR => [
                Self::translation_func(EdgeDirection::BTT, 0, right_region),
                Self::translation_func(EdgeDirection::RTL, height, top_region),
                Self::translation_func(EdgeDirection::TTB, height + width, left_region),
                Self::translation_func(EdgeDirection::LTR, height + width + height, bottom_region),
            ],
            StartCorner::TR => [
                Self::translation_func(EdgeDirection::RTL, 0, top_region),
                Self::translation_func(EdgeDirection::TTB, width, left_region),
                Self::translation_func(EdgeDirection::LTR, width + height, bottom_region),
                Self::translation_func(EdgeDirection::BTT, width + height + width, right_region),
            ],
        }
    }

    fn translation_func(direction: EdgeDirection, offset: i32, region: Rect) -> Action {
        debug!("Creating translation func for direction {:?}", direction);
        match direction {
            // Read the roi from right to left while calculating the mean and writing to target
            EdgeDirection::RTL => {
                Box::new(move |source: &Mat, target: &mut Mat| -> Result<()> {
                    let roi = Mat::roi(source, region)?;

                    // Offset + target index will be the actual index of a new value
                    let mut target_index = 0;

                    // Iterate over the roi from right to left
                    for col in (0..roi.cols()).rev() {
                        // Will keep the mean RGB values
                        let mut mean_b = 0;
                        let mut mean_g = 0;
                        let mut mean_r = 0;

                        // Add up the values in one column up
                        for row in 0..roi.rows() {
                            let pixel = roi.at_2d::<Vec3b>(row, col)?;

                            mean_b += pixel[0] as u32;
                            mean_g += pixel[1] as u32;
                            mean_r += pixel[2] as u32;
                        }

                        // Calculate the mean
                        mean_b /= roi.rows() as u32;
                        mean_g /= roi.rows() as u32;
                        mean_r /= roi.rows() as u32;

                        for row in 0..target.rows() {
                            // Write resulting RGB value to target and increase counter
                            *target.at_2d_mut(row, offset + target_index)? =
                                Vec3b::from_array([mean_b as u8, mean_g as u8, mean_r as u8]);
                        }
                        target_index += 1;
                    }

                    Ok(())
                })
            }
            EdgeDirection::LTR => Box::new(move |source: &Mat, target: &mut Mat| -> Result<()> {
                let roi = Mat::roi(source, region)?;

                let mut target_index = 0;

                // Iterate over the roi from right to left
                for col in 0..roi.cols() {
                    // Will keep the mean RGB values
                    let mut mean_b = 0;
                    let mut mean_g = 0;
                    let mut mean_r = 0;

                    for row in 0..roi.rows() {
                        let pixel = roi.at_2d::<Vec3b>(row, col)?;

                        mean_b += pixel[0] as u32;
                        mean_g += pixel[1] as u32;
                        mean_r += pixel[2] as u32;
                    }

                    // Calculate the mean
                    mean_b /= roi.rows() as u32;
                    mean_g /= roi.rows() as u32;
                    mean_r /= roi.rows() as u32;

                    for row in 0..target.rows() {
                        // Write resulting RGB value to target and increase counter
                        *target.at_2d_mut(row, offset + target_index)? =
                            Vec3b::from_array([mean_b as u8, mean_g as u8, mean_r as u8]);
                    }
                    target_index += 1;
                }

                Ok(())
            }),
            EdgeDirection::TTB => Box::new(move |source: &Mat, target: &mut Mat| -> Result<()> {
                let roi = Mat::roi(source, region)?;

                let mut target_index = 0;

                // Iterate over the roi from right to left
                for row in 0..roi.rows() {
                    // Will keep the mean RGB values
                    let mut mean_b = 0;
                    let mut mean_g = 0;
                    let mut mean_r = 0;

                    for col in 0..roi.cols() {
                        let pixel = roi.at_2d::<Vec3b>(row, col)?;

                        mean_b += pixel[0] as u32;
                        mean_g += pixel[1] as u32;
                        mean_r += pixel[2] as u32;
                    }

                    // Calculate the mean
                    mean_b /= roi.cols() as u32;
                    mean_g /= roi.cols() as u32;
                    mean_r /= roi.cols() as u32;

                    for row in 0..target.rows() {
                        // Write resulting RGB value to target and increase counter
                        *target.at_2d_mut(row, offset + target_index)? =
                            Vec3b::from_array([mean_b as u8, mean_g as u8, mean_r as u8]);
                    }
                    target_index += 1;
                }

                Ok(())
            }),
            EdgeDirection::BTT => Box::new(move |source: &Mat, target: &mut Mat| -> Result<()> {
                let roi = Mat::roi(source, region)?;

                let mut target_index = 0;

                // Iterate over the roi from right to left
                for row in (0..roi.rows()).rev() {
                    // Will keep the mean RGB values
                    let mut mean_b = 0;
                    let mut mean_g = 0;
                    let mut mean_r = 0;

                    for col in 0..roi.cols() {
                        let pixel = roi.at_2d::<Vec3b>(row, col)?;

                        mean_b += pixel[0] as u32;
                        mean_g += pixel[1] as u32;
                        mean_r += pixel[2] as u32;
                    }

                    // Calculate the mean
                    mean_b /= roi.cols() as u32;
                    mean_g /= roi.cols() as u32;
                    mean_r /= roi.cols() as u32;

                    for row in 0..target.rows() {
                        // Write resulting RGB value to target and increase counter
                        *target.at_2d_mut(row, offset + target_index)? =
                            Vec3b::from_array([mean_b as u8, mean_g as u8, mean_r as u8]);
                    }
                    target_index += 1;
                }

                Ok(())
            }),
        }
    }
}
