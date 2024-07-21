use anyhow::Result;

use tracing::debug;

use crate::{
    lightstrip::{self, Lightstrip},
    settings::{Direction, StartCorner},
};

// Roi, Target Mat, Offset
type Action = Box<dyn Fn(&[(u32, u32, u32)], &Lightstrip)>;

#[derive(Debug)]
enum EdgeDirection {
    RTL,
    LTR,
    TTB,
    BTT,
}

struct Rect {
    sx: usize,
    sy: usize,
    w: usize,
    h: usize,
}

pub struct TranslationEngine {}

impl TranslationEngine {
    /// An array with 4 closures will be returned by this function. These 4 closures will
    /// correspons to the 4 edges of a display. The 4 closures will be applied to each incoming
    /// frame and translate the color values to a 1D array that represents the LED strip
    pub fn new(
        start: StartCorner,
        direction: Direction,
        width: i32,
        height: i32,
        thickness: i32,
        fw: i32,
        fh: i32,
        ppl: i32,
    ) -> [Action; 4] {
        debug!(
            "Setting up frame translation for start: {:?} direction: {:?} border_thickness: {}",
            start, direction, thickness
        );
        match direction {
            Direction::CW => {
                Self::get_translation_funcs_cw(start, width, height, thickness, fw, fh, ppl)
            }
            Direction::CCW => {
                Self::get_translation_funcs_ccw(start, width, height, thickness, fw, fh, ppl)
            }
        }
    }

    /// Creates an array of exactly four functions that later shall be applied on an input frame.
    /// There are four possible cases depending on which corner the led_strip starts at. This gives
    /// the functions for clockwise.
    fn get_translation_funcs_cw(
        start: StartCorner,
        rect_width: i32,
        rect_height: i32,
        thickness: i32,
        fw: i32,
        fh: i32,
        ppl: i32,
    ) -> [Action; 4] {
        debug!(
            "Setting up translation functions for clockwise layout starting from {:?}",
            start
        );
        let top_region = Rect {
            sx: 0,
            sy: 0,
            w: rect_width,
            h: thickness,
        };
        let right_region = Rect {
            sx: rect_width,
            sy: 0,
            w: thickness,
            h: rect_height,
        };
        let bottom_region = Rect {
            sx: thickness,
            sy: rect_height,
            w: rect_width,
            h: thickness,
        };
        let left_region = Rect {
            sx: 0,
            sy: thickness,
            w: thickness,
            h: rect_height,
        };

        match start {
            StartCorner::TL => [
                Self::translation_func(EdgeDirection::LTR, top_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::TTB, right_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::RTL, bottom_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::BTT, left_region, fw, fh, ppl),
            ],
            StartCorner::TR => [
                Self::translation_func(EdgeDirection::TTB, right_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::RTL, bottom_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::BTT, left_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::LTR, top_region, fw, fh, ppl),
            ],
            StartCorner::BR => [
                Self::translation_func(EdgeDirection::RTL, bottom_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::BTT, left_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::LTR, top_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::TTB, right_region, fw, fh, ppl),
            ],
            StartCorner::BL => [
                Self::translation_func(EdgeDirection::BTT, left_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::LTR, top_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::TTB, right_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::RTL, bottom_region, fw, fh, ppl),
            ],
        }
    }

    /// Creates an array of exactly four functions that later shall be applied on an input frame.
    /// There are four possible cases depending on which corner the led_strip starts at. This gives
    /// the functions for counter clockwise.
    fn get_translation_funcs_ccw(
        start: StartCorner,
        rect_width: i32,
        rect_height: i32,
        thickness: i32,
        fw: i32,
        fh: i32,
        ppl: i32,
    ) -> [Action; 4] {
        debug!(
            "Setting up translation functions for counter clockwise layout starting from {:?}",
            start
        );
        let top_region = Rect {
            sx: thickness,
            sy: 0,
            w: rect_width,
            h: thickness,
        };
        let right_region = Rect {
            sx: rect_width,
            sy: thickness,
            w: thickness,
            h: rect_height,
        };
        let bottom_region = Rect {
            sx: 0,
            sy: rect_height,
            w: rect_width,
            h: thickness,
        };
        let left_region = Rect {
            sx: 0,
            sy: 0,
            w: thickness,
            h: rect_height,
        };

        match start {
            StartCorner::TL => [
                Self::translation_func(EdgeDirection::TTB, left_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::LTR, bottom_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::BTT, right_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::RTL, top_region, fw, fh, ppl),
            ],
            StartCorner::BL => [
                Self::translation_func(EdgeDirection::LTR, bottom_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::BTT, right_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::RTL, top_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::TTB, left_region, fw, fh, ppl),
            ],
            StartCorner::BR => [
                Self::translation_func(EdgeDirection::BTT, right_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::RTL, top_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::TTB, left_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::LTR, bottom_region, fw, fh, ppl),
            ],
            StartCorner::TR => [
                Self::translation_func(EdgeDirection::RTL, top_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::TTB, left_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::LTR, bottom_region, fw, fh, ppl),
                Self::translation_func(EdgeDirection::BTT, right_region, fw, fh, ppl),
            ],
        }
    }

    /// Returns a closure translation function that will can be applied to an incoming frame. Each
    /// translation function averages the values in the provided region along the specified
    /// direction. The resulting values will be written to target starting from an offset.
    fn translation_func(
        direction: EdgeDirection,
        region: Rect,
        fw: i32,
        fh: i32,
        ppl: i32,
    ) -> Action {
        debug!("Creating translation func for direction {:?}", direction);
        match direction {
            // Read the roi from right to left while calculating the mean and writing to target
            EdgeDirection::RTL => {
                Box::new(move |source: &[(u32, u32, u32)], lightstrip: &Lightstrip| {
                    let row = source[region.sy * fw + 1..(region.sy + 1) * fw];

                    for chunk in row.chunks_exact(ppl) {
                        // Will keep the mean RGB values
                        let mut mean_r: u32 = 0;
                        let mut mean_g: u32 = 0;
                        let mut mean_b: u32 = 0;

                        for value in chunk.iter() {
                            mean_r += value.0;
                            mean_g += value.1;
                            mean_b += value.2;
                        }

                        mean_r /= ppl;
                        mean_g /= ppl;
                        mean_b /= ppl;

                        lightstrip.set((mean_r as u8, mean_g as u8, mean_b as u8));
                        lightstrip.next();
                    }
                })
            }
            EdgeDirection::LTR => {
                Box::new(move |source: &[(u32, u32, u32)], lightstrip: &Lightstrip| {
                    let row = source[region.sy * fw..((region.sy + 1) * fw) - 1];
                    for chunk in row.chunks_exact(ppl) {
                        let mut mean_r: u32 = 0;
                        let mut mean_g: u32 = 0;
                        let mut mean_b: u32 = 0;

                        for value in chunk.iter() {
                            mean_r += value.0;
                            mean_g += value.1;
                            mean_b += value.2;
                        }

                        mean_r /= ppl;
                        mean_g /= ppl;
                        mean_b /= ppl;

                        lightstrip.set((mean_r as u8, mean_g as u8, mean_b as u8));
                        lightstrip.next();
                    }
                })
            }
            EdgeDirection::TTB => {
                Box::new(move |source: &[(u32, u32, u32)], lightstrip: &Lightstrip| {
                    let mut mean_r: u32 = 0;
                    let mut mean_g: u32 = 0;
                    let mut mean_b: u32 = 0;

                    for row in 0..((region.h / ppl) * ppl) {
                        let pixel = source[row * fw + region.x];

                        mean_r += pixel.0;
                        mean_g += pixel.1;
                        mean_b += pixel.2;

                        if row % ppl == (ppl - 1) {
                            mean_r /= ppl;
                            mean_g /= ppl;
                            mean_b /= ppl;

                            lightstrip.set((mean_r, mean_g, mean_b));
                            lightstrip.next();
                        }
                    }
                })
            }
            EdgeDirection::BTT => {
                Box::new(move |source: &[(u32, u32, u32)], lightstrip: &Lightstrip| {
                    let mut mean_r: u32 = 0;
                    let mut mean_g: u32 = 0;
                    let mut mean_b: u32 = 0;

                    // If the height of the region is 10 and we have 3 pixels_per_led the last
                    // pixel of the region wont be used. The expression means that it shall
                    // start at the index so that pixel_per_led perfectly fits into the
                    // iterator x times
                    for (i, row) in ((fh - ((region.h / ppl) * ppl))..fh).rev().enumerate() {
                        let pixel = source[row * fw + region.x];

                        mean_r += pixel.0;
                        mean_g += pixel.1;
                        mean_b += pixel.2;

                        if i % ppl == (ppl - 1) {
                            mean_r /= ppl;
                            mean_g /= ppl;
                            mean_b /= ppl;

                            lightstrip.set((mean_r as u8, mean_g as u8, mean_b as u8));
                            lightstrip.next();
                        }
                    }
                })
            }
        }
    }
}
