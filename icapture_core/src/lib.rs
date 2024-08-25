pub mod config;
pub mod device;

pub struct ICaptureCore {
}

impl ICaptureCore {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn find_device_by_name(name: &str) -> i32 {
        device::get_capture_device_id_by_name(name).unwrap_or_default()
    }

    pub fn init_device(index: i32) -> Result<opencv::videoio::VideoCapture, opencv::Error> {
        opencv::videoio::VideoCapture::new(index, opencv::videoio::CAP_DSHOW)
    }

    pub fn dispose_device(_camera: opencv::videoio::VideoCapture) {
    }
}
