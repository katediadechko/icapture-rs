use opencv::{videoio, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Codec {
    H264,
    MJPG,
    WMV3,
}

impl Codec {
    pub fn fourcc(&self) -> Result<i32> {
        match self {
            Codec::H264 => videoio::VideoWriter::fourcc('H', '2', '6', '4'), // OK            
            Codec::MJPG => videoio::VideoWriter::fourcc('M', 'J', 'P', 'G'), // NOK at 60 fps
            Codec::WMV3 => videoio::VideoWriter::fourcc('W', 'M', 'V', '3'), // NOK at 60 fps
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Codec::H264 => "mp4",
            Codec::MJPG => "avi",
            Codec::WMV3 => "wmv",
        }
    }
}
