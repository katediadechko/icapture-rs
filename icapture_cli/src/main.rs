use clap::Parser;
use icapture_core::{capture::*, config::*};
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".\\config.json")]
    config_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().format_timestamp_millis().init();

    let args = Args::parse();

    let config = Config::new(&args.config_file);
    let mut capture = Capture::new(&config)?;
    capture.start_grab_video()?;
    thread::sleep(Duration::from_millis(10000));
    capture.stop_grab_video()?;
    capture.dispose()?;

    Ok(())
}
