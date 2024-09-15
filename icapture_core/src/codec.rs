use opencv::{videoio, Result};

#[derive(Clone, Copy, Debug)]
pub enum Codec {
    MJPG,
    H264,
    MP4V,
    WMV3,
    X264,
    DIV3,
    DIVX,
    AVC1,
}

impl Codec {
    pub fn fourcc(&self) -> Result<i32> {
        match self {
            Codec::MJPG => videoio::VideoWriter::fourcc('M', 'J', 'P', 'G'), // NOK - 60 fps
            Codec::H264 => videoio::VideoWriter::fourcc('H', '2', '6', '4'), // OK
            Codec::MP4V => videoio::VideoWriter::fourcc('M', 'P', '4', 'V'), // NOK - 0 bytes
            Codec::WMV3 => videoio::VideoWriter::fourcc('W', 'M', 'V', '3'), // NOK - 60 fps
            Codec::X264 => videoio::VideoWriter::fourcc('X', '2', '6', '4'), // OK
            Codec::DIV3 => videoio::VideoWriter::fourcc('D', 'I', 'V', '3'), // NOK - 0 bytes
            Codec::DIVX => videoio::VideoWriter::fourcc('D', 'I', 'V', 'X'), // NOK - 0 bytes
            Codec::AVC1 => videoio::VideoWriter::fourcc('A', 'V', 'C', '1'), // OK
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Codec::MJPG => "avi",
            Codec::H264 | Codec::X264 | Codec::AVC1 => "mp4",
            Codec::MP4V => "mp4",
            Codec::WMV3 => "wmv",
            Codec::DIV3 | Codec::DIVX => "avi",
        }
    }
}
