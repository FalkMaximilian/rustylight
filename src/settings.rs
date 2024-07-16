use std::{env, fs, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Direction {
    CW,
    CCW,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum StartCorner {
    TL,
    TR,
    BL,
    BR,
}

/// Compatibility loglevel enum because LevelFilter does not implement Serialize/Deserialize
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
}

// Implement From trait to convert from LogLevel to LevelFilter
impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> LevelFilter {
        match level {
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}

/// Resolution will be used to set the internal processing resolution. It is likely not necessary
/// to process each frame in 1080p or more if only 100-200 pixels are needed for the lightstrip.
/// This also reduces load on the system running rustylight. This is useful because usually low
/// powered devices will be used for a diy ambilight setup.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Resolution {
    FHD,
    HD,
    VGA,
}

// Convert to pixel resolution
impl From<Resolution> for (f64, f64) {
    fn from(res: Resolution) -> Self {
        match res {
            Resolution::FHD => (1920.0, 1080.0),
            Resolution::HD => (1280.0, 720.0),
            Resolution::VGA => (640.0, 480.0),
        }
    }
}

/// Settings for rustylight that will be read from settings.toml file
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub log_level: LogLevel,
    pub video_device: i32,
    pub capture_area_size: i32,
    pub processing_resolution: Resolution,
    pub start_corner: StartCorner,
    pub direction: Direction,
    pub led_count: i32,
}

impl Settings {
    /// Read settings.toml file or create a new one with default values if it doesn't exist.
    pub fn new() -> Result<Self> {
        // Setup path to settings file
        let home_dir = env::var("HOME").expect("Could not find the HOME environment variable");
        let mut settings_path = PathBuf::from(&home_dir);
        settings_path.push(".config/rustylight/settings.toml");
        println!("Attemting to read settings from {:?}", settings_path);

        if settings_path.exists() {
            let settings_str = fs::read_to_string(settings_path)?;
            let settings: Settings = toml::from_str(&settings_str)?;

            println!("Successfully read settings from file!");
            Ok(settings)
        } else {
            println!("Config file does not exist at {:?}", settings_path);

            if let Some(parent_path) = settings_path.parent() {
                fs::create_dir_all(parent_path)?;
            }

            let settings = Settings::default();
            let toml = toml::to_string(&settings)?;
            fs::write(settings_path, toml)?;
            println!("Successfully created settings file!");
            Ok(settings)
        }
    }

    /// Create default settings. They can be changed later in the file.
    fn default() -> Settings {
        Settings {
            log_level: LogLevel::Info,
            video_device: 0,
            capture_area_size: 10,
            processing_resolution: Resolution::VGA,
            start_corner: StartCorner::BL,
            direction: Direction::CW,
            led_count: 123,
        }
    }
}
