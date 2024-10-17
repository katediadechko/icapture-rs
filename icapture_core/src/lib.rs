//! # icapture_core
//!
//! `icapture_core` is a library for capturing images and video on Windows.

pub use self::capture::Capture;
pub use self::capture::CaptureError;
pub use self::capture::codec;
pub use self::capture::device;
pub use self::config::Config;

pub mod capture;
pub mod config;
