use icapture_core::{capture::*, config::*};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_file("config.json");
    let mut capture = Capture::new(&config.device_name)?;

    capture.preview()?;

    Ok(())
}
