use chrono::Local;
use log::debug;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn create_dir(dir_path: &str) -> Result<()> {
    let path = Path::new(dir_path);
    if !path.exists() {
        fs::create_dir_all(path)?;
        debug!("directory created: {}", dir_path);
    } else {
        debug!("directory already exists: {}", dir_path);
    }
    Ok(())
}

pub fn get_name(extension: &str) -> String {
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S.%3f").to_string();
    format!("{}.{}", timestamp, extension)
}
