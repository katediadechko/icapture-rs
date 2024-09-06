use icapture_core::{capture::*, config::*};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::from_file("config.json");

    let mut capture = Capture::new(&config)?;
    capture.preview()?;
    capture.grab_frame()?;
    capture.dispose()?;

    Ok(())
}
