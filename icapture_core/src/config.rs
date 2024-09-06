use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub device_name: String,
    pub fps: u8,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
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
            device_name: String::from("USB Capture HDMI 4K+"),
            fps: 30,
        }
    }
}

impl Config {
    pub fn from_file(file_path: &str) -> Self {
        match Self::try_from_file(file_path) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Cannot read config file '{file_path}'");
                eprintln!("Falling back to default configuration {:?}", Self::default());
                Self::default()
            }
        }
    }

    fn try_from_file(file_path: &str) -> Result<Self, ConfigError> {
        let reader = BufReader::new(File::open(file_path)?);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
