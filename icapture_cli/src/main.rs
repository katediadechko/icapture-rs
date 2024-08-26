use icapture_core::ICapture;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut icapture = ICapture::new("USB Capture HDMI 4K+")?;

    icapture.preview()?;

    Ok(())
}
