//! Provides FourCC values and file extentions for different video codecs.

use opencv::{videoio, Result};
use serde::{Deserialize, Serialize};

/// Defines supported video codecs.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Codec {
    /// Advanced Video Coding
    H264,
    /// Motion JPEG
    MJPG,
    /// Windows Media Video
    WMV3,
}

impl Codec {
    /// Gets a FourCC value of a specific video codec.
    /// 
    /// # Errors
    /// 
    /// Returns the corresponding OpenCV error in case of failure.
    pub fn fourcc(&self) -> Result<i32> {
        match self {
            Codec::H264 => videoio::VideoWriter::fourcc('H', '2', '6', '4'), // OK            
            Codec::MJPG => videoio::VideoWriter::fourcc('M', 'J', 'P', 'G'), // NOK at 60 fps
            Codec::WMV3 => videoio::VideoWriter::fourcc('W', 'M', 'V', '3'), // NOK at 60 fps
        }
    }

    /// Gets a file extention for a specific video codec.
    pub fn file_extension(&self) -> &'static str {
        match self {
            Codec::H264 => "mp4",
            Codec::MJPG => "avi",
            Codec::WMV3 => "wmv",
        }
    }
}
