use clap::{Parser, Subcommand};
use icapture_core::{capture::*, config::*};
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".\\config.json")]
    config_file: String,
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Grab a single frame
    GrabFrame,

    /// Grab a video
    GrabVideo {
        /// Duration of the video in seconds
        #[arg(short, long)]
        duration: u32,
    },

    /// List available devices
    ListDevices,

    /// Preview the camera feed
    Preview,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder().format_timestamp_millis().init();

    let args = Args::parse();

    let config = Config::new(&args.config_file);

    match args.action {
        Action::GrabFrame => {
            let mut capture = Capture::new(&config)?;
            capture.grab_frame()?;
            capture.dispose()?;
        }
        Action::GrabVideo { duration } => {
            let mut capture = Capture::new(&config)?;
            capture.start_grab_video()?;
            thread::sleep(Duration::from_secs(duration as u64));
            capture.stop_grab_video()?;
            capture.dispose()?;
        }
        Action::ListDevices => println!("{:?}", device::enumerate_capture_devices()?),
        Action::Preview => {
            let mut capture = Capture::new(&config)?;
            capture.preview()?;
            capture.dispose()?;
        }
    }

    Ok(())
}
