use icapture_core::{capture::*, config::*};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::from_file("config.json");

    let mut capture = Capture::new(&config)?;
    capture.set_fps(0)?;
    capture.set_frame_size((0, 0))?;
    capture.preview()?;
    capture.dispose()?;

    Ok(())
}
