use clap::Parser;
use clap::{arg, ValueEnum};

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
            StartCorner::TL => println!("Top left corner selected"),
            StartCorner::TR => println!("Top right corner selected"),
            StartCorner::BL => println!("Bottom left corner selected"),
            StartCorner::BR => println!("Bottom right corner selected"),
        }

        match cli.direction {
            Direction::CW => println!("Clockwise selected"),
            Direction::CCW => println!("Counter Clockwise selected"),
        }

        println!("You have specified that there are {} LEDs", cli.led_count);

        cli
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Direction {
    /// Clockwise
    CW,
    /// Counter Clockwise
    CCW,
}
