use opencv::{
    highgui,
    prelude::*,
    videoio::{self, VideoCapture, VideoCaptureTraitConst},
    Result,
};

pub mod device;

pub struct ICapture {
    device_name: String,
    instance: VideoCapture,
}

impl ICapture {
    pub fn new(name: &str) -> Result<Self, String> {
        let device_name: String = name.to_string();
        let device_id = match Self::find_device_by_name(name) {
            Some(id) => id as i32,
            None => return Err(format!("Cannot find capture device '{name}'")),
        };

        let instance = match Self::init_device(device_id) {
            Ok(instance) => instance,
            Err(error) => return Err(error.message),
        };

        match instance.is_opened() {
            Ok(is_opened) => {
                if !is_opened {
                    return Err(format!("Cannot open capture device '{name}'"));
                }
            }
            Err(error) => {
                return Err(error.message);
            }
        };

        Ok(Self { device_name, instance })
    }

    pub fn dispose(_instance: &VideoCapture) {
    }

    pub fn preview(&mut self) -> Result<()> {
        let window: &str = self.device_name.as_str();
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
        loop {
            let mut frame = Mat::default();
            self.instance.read(&mut frame)?;
            if frame.size()?.width > 0 {
                highgui::imshow(window, &frame)?;
            }
            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
        Ok(())
    }

    fn find_device_by_name(name: &str) -> Option<usize> {
        let devices = device::enumerate_capture_devices().ok()?;
        device::get_capture_device_id_by_name(&devices, name)
    }

    fn init_device(device_id: i32) -> Result<VideoCapture, opencv::Error> {
        VideoCapture::new(device_id, videoio::CAP_DSHOW)
    }
}
