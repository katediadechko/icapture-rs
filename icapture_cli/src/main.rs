use icapture_core::{capture::*, config::*};
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().format_timestamp_millis().init();

    let thread1 = thread::spawn(|| -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = Config::new("config.json");
        let mut capture = Capture::new(&config)?;
        capture.start_grab_video()?;
        thread::sleep(Duration::from_millis(10000));
        capture.stop_grab_video()?;
        capture.dispose()?;
        Ok(())
    });
/*
    let thread2 = thread::spawn(|| -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = Config::new("config.json");
        let mut capture = Capture::new(&config)?;
        thread::sleep(Duration::from_millis(500));
        let _ = capture.grab_frame(); // returns error
        thread::sleep(Duration::from_millis(2500));
        _ = capture.preview(); // return error
        capture.dispose()?;
        Ok(())
    });
*/
    let _ = thread1.join().unwrap();
//    let _ = thread2.join().unwrap();

    Ok(())
}
