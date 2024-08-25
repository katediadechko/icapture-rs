use icapture_core::ICapture;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let icapture = ICapture::new("USB Capture HDMI 4K+")?;

    Ok(())
}
