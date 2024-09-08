use crate::{config::Config, device, file};
use log::{debug, warn, error};
use opencv::{core, highgui, imgcodecs, prelude::*, videoio::*, Error, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("cannot create file or directory '{0}'")]
    CreateFileDirectory(String),
    #[error("cannot find capture device '{0}'")]
    DeviceNotFound(String),
    #[error("cannot open capture device '{0}'")]
    DeviceOpenError(String),
    #[error("cannot grab frame")]
    FrameError,
    #[error("opencv error: {0}")]
    OpenCvError(#[from] Error),
}

pub struct Capture {
    config: Config,
    instance: VideoCapture,
}

impl Capture {
    pub fn new(conf: &Config) -> Result<Self, CaptureError> {
        let config = conf.clone();
        let device_name = &config.device_name;
        let data_dir = &config.data_dir;
        if file::create_dir(data_dir).is_err() {
            let err = CaptureError::CreateFileDirectory(data_dir.clone());
            error!("{}", err);
            return Err(err);
        }

        let device_id = Self::capture_find_device_by_name(device_name)
            .ok_or_else(|| CaptureError::DeviceNotFound(device_name.clone()))?;

        let mut instance = Self::capture_new_device(device_id)?;

        if !instance.is_opened()? {
            let err = CaptureError::DeviceOpenError(device_name.clone());
            error!("{}", err);
            return Err(err);
        }

        Self::capture_set_fps(&mut instance, conf.fps)?;
        Self::capture_verify_fps(&instance, conf.fps)?;
        Self::capture_set_frame_size(&mut instance, (conf.frame_width, conf.frame_height))?;
        Self::capture_verify_frame_size(&instance, (conf.frame_width, conf.frame_height))?;

        Ok(Self {
            config,
            instance,
        })
    }

    pub fn dispose(&mut self) -> Result<()> {
        self.instance.release()
    }

    pub fn preview(&mut self) -> Result<()> {
        let window: &str = &self.config.device_name;
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
        loop {
            let mut frame = Mat::default();
            self.instance.read(&mut frame)?;
            if frame.size()?.width > 0 {
                highgui::imshow(window, &frame)?;
            }
            let key = highgui::wait_key(10)?;
            if key == 27 || highgui::get_window_property(window, highgui::WND_PROP_VISIBLE)? < 1.0 {
                break;
            }
        }
        Ok(())
    }

    pub fn grab_frame_to_file(&mut self, file_path: &str) -> Result<bool, CaptureError> {
        debug!("grab frame to file '{}'", file_path);
        let mut frame = Mat::default();
        let success = self.instance.read(&mut frame)?;
        if !success || frame.empty() {
            let err = CaptureError::FrameError;
            error!("{}", err);
            return Err(err);
        }
        let mut params = core::Vector::default();
        params.push(imgcodecs::IMWRITE_PNG_COMPRESSION);
        params.push(0);
        imgcodecs::imwrite(file_path, &frame, &params)?;
        Ok(success)
    }

    pub fn grab_frame(&mut self) -> Result<bool, CaptureError> {
        let file_path = format!("{}\\{}", &self.config.data_dir, file::get_name("png"));
        self.grab_frame_to_file(&file_path)
    }

    pub fn get_fps(&self) -> Result<u32, CaptureError> {
        Self::capture_get_fps(&self.instance).map_err(CaptureError::from)
    }

    pub fn get_frame_size(&self) -> Result<(u32, u32), CaptureError> {
        Self::capture_get_frame_size(&self.instance).map_err(CaptureError::from)
    }

    pub fn set_fps(&mut self, fps: u32) -> Result<bool, CaptureError> {
        Self::capture_set_fps(&mut self.instance, fps).map_err(CaptureError::from)?;
        self.config.fps = fps;
        Self::capture_verify_fps(&self.instance, fps).map_err(CaptureError::from)
    }

    pub fn set_frame_size(&mut self, size: (u32, u32)) -> Result<bool, CaptureError> {
        Self::capture_set_frame_size(&mut self.instance, size).map_err(CaptureError::from)?;
        self.config.frame_width = size.0;
        self.config.frame_height = size.1;
        Self::capture_verify_frame_size(&self.instance, size).map_err(CaptureError::from)
    }

    fn capture_find_device_by_name(name: &str) -> Option<u32> {
        let devices = device::enumerate_capture_devices().ok()?;
        device::get_capture_device_id_by_name(&devices, name)
    }

    fn capture_new_device(device_id: u32) -> Result<VideoCapture, opencv::Error> {
        VideoCapture::new(device_id as i32, CAP_MSMF)
    }

    fn capture_get_fps(capture: &VideoCapture) -> Result<u32, opencv::Error> {
        let fps = capture.get(CAP_PROP_FPS).map(|fps| fps as u32)?;
        debug!("get fps: {fps}");
        Ok(fps)
    }

    fn capture_get_frame_size(capture: &VideoCapture) -> Result<(u32, u32), opencv::Error> {
        let width = capture.get(CAP_PROP_FRAME_WIDTH).map(|w| w as u32)?;
        let height = capture.get(CAP_PROP_FRAME_HEIGHT).map(|h| h as u32)?;
        debug!("get frame size: {width}x{height}");
        Ok((width, height))
    }

    fn capture_set_fps(capture: &mut VideoCapture, fps: u32) -> Result<bool, opencv::Error> {
        let fps_set = capture.set(CAP_PROP_FPS, fps as f64)?;
        debug!("set fps: {fps}");
        Ok(fps_set)
    }

    fn capture_set_frame_size(
        capture: &mut VideoCapture,
        size: (u32, u32),
    ) -> Result<bool, opencv::Error> {
        let width_set = capture.set(CAP_PROP_FRAME_WIDTH, size.0 as f64)?;
        let height_set = capture.set(CAP_PROP_FRAME_HEIGHT, size.1 as f64)?;
        debug!("set frame size: {}x{}", size.0, size.1);
        Ok(width_set && height_set)
    }

    fn capture_verify_fps(
        capture: &VideoCapture,
        expected_fps: u32,
    ) -> Result<bool, opencv::Error> {
        let actual_fps = Self::capture_get_fps(capture)?;
        let success = actual_fps == expected_fps;
        if !success {
            warn!(
                "fps mismatch: expected {}, actual {}",
                expected_fps, actual_fps
            )
        }
        Ok(success)
    }

    fn capture_verify_frame_size(
        capture: &VideoCapture,
        expected_size: (u32, u32),
    ) -> Result<bool, opencv::Error> {
        let actual_size = Self::capture_get_frame_size(capture)?;
        let success = actual_size == expected_size;

        if !success {
            warn!(
                "frame size mismatch: expected {:?}, actual {:?}",
                expected_size, actual_size
            )
        }
        Ok(success)
    }
}
