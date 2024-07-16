use clap::Parser;
use clap::{arg, ValueEnum};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Parser)]
#[command(name = "rustylight")]
#[command(author = "Maximilian Falk")]
#[command(version = "0.1")]
#[command(
    about = "Diy ambilight implementation",
    long_about = "Takes video data and maps the edges onto an LED strip"
)]
pub struct RustylightCli {
    #[arg(short, long, value_enum, required = true)]
    pub start_corner: StartCorner,

    #[arg(short, long, value_enum, required = true)]
    pub direction: Direction,

    #[arg(short, long, required = true)]
    pub led_count: u32,
}

impl RustylightCli {
    pub fn setup() -> RustylightCli {
        let cli = RustylightCli::parse();

        match cli.start_corner {
            StartCorner::TL => info!("Top left corner was selected as start"),
            StartCorner::TR => info!("Top right corner was selected as start"),
            StartCorner::BL => info!("Bottom left corner was selected as start"),
            StartCorner::BR => info!("Bottom right corner was selected as start"),
        }

        match cli.direction {
            Direction::CW => info!("Clockwise was selected"),
            Direction::CCW => info!("Counter Clockwise was selected"),
        }

        info!("You specified that there are {} LEDs", cli.led_count);

        cli
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
pub enum StartCorner {
    /// Top Left Corner
    TL,
    /// Top Right Corner
    TR,
    /// Bottom Left Corner
    BL,
    /// Bottom Right Corner
    BR,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
pub enum Direction {
    /// Clockwise
    CW,
    /// Counter Clockwise
    CCW,
}
