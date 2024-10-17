//! Provides operations for serializing a capturing device configuration.

use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::BufReader;

use crate::capture::codec::Codec;

/// Defines a configuration object.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Config {
    /// Capturing device index
    pub device_id: u32,
    /// Desired FPS
    pub fps: u32,
    /// Desired frame width
    pub frame_width: u32,
    /// Desired frame height
    pub frame_height: u32,
    /// Path to store files at
    pub data_dir: String,
    /// Desired codec for saving video
    pub codec: Codec,
}

/// Defines possible serialization errors.
#[derive(Debug)]
pub enum ConfigError {
    /// I/O error
    Io(io::Error),
    /// JSON error
    Json(serde_json::Error),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> ConfigError {
        ConfigError::Json(err)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            device_id: 0,
            fps: 30,
            frame_width: 1920,
            frame_height: 1080,
            data_dir: String::from("c:\\icapture_data"),
            codec: Codec::H264,
        }
    }
}

impl Config {
    /// Constructor for a configuration object.
    pub fn new(file_path: &str) -> Self {
        match Self::load_from_file(file_path) {
            Ok(config) => {
                debug!("using config {:?}", &config);
                config
            }
            Err(_) => {
                warn!("cannot read config file '{file_path}'");
                warn!(
                    "falling back to default config {:?}",
                    Self::default()
                );
                Self::default()
            }
        }
    }

    fn load_from_file(file_path: &str) -> Result<Self, ConfigError> {
        let reader = BufReader::new(File::open(file_path)?);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_from_valid_file_succeeds() {
        let config = Config {
            device_id: 7,
            fps: 42,
            frame_width: 2560,
            frame_height: 1440,
            data_dir: "test directory".to_string(),
            codec: Codec::H264,
        };
        let json = serde_json::to_string(&config).unwrap();
        let file_path = "test_config.json";
        fs::write(file_path, json).unwrap();
        let loaded_config = Config::new(file_path);
        assert_eq!(loaded_config, config);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_from_invalid_file_defaults() {
        let file_path = "invalid_test_config.json";
        let loaded_config = Config::new(file_path);
        assert_eq!(loaded_config, Config::default());
    }
}
