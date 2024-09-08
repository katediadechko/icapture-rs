use icapture_core::{capture::*, config::*};
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().format_timestamp_millis().init();

    let thread1 = thread::spawn(|| -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = Config::from_file("config.json");
        let mut capture = Capture::new(&config)?;
        capture.grab_frame()?;
        thread::sleep(Duration::from_millis(5000));
        capture.grab_frame()?;
        capture.dispose()?;
        Ok(())
    });

    let thread2 = thread::spawn(|| -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = Config::from_file("config.json");
        let mut capture = Capture::new(&config)?;
        capture.grab_frame()?;
        capture.dispose()?;
        Ok(())
    });

    let _ = thread1.join().unwrap();
    let _ = thread2.join().unwrap();

    Ok(())
}
