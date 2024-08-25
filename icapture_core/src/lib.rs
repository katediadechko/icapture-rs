use opencv::{videoio, videoio::VideoCapture, Result};

pub mod device;

pub struct ICapture {
    instance: VideoCapture,
}

impl ICapture {
    pub fn new(name: &str) -> Result<Self, String> {
        let device_id = match Self::find_device_by_name(name) {
            Some(id) => id as i32,
            None => return Err(format!("Cannot find capture device '{name}'")),
        };

        let instance = match Self::init_device(device_id) {
            Ok(instance) => instance,
            Err(error) => return Err(error.message),
        };

        Ok(Self { instance })
    }

    pub fn dispose(_instance: &VideoCapture) {}

    fn find_device_by_name(name: &str) -> Option<usize> {
        let devices = device::enumerate_capture_devices().ok()?;
        device::get_capture_device_id_by_name(&devices, name)
    }

    fn init_device(device_id: i32) -> Result<VideoCapture, opencv::Error> {
        VideoCapture::new(device_id, videoio::CAP_DSHOW)
    }
}
