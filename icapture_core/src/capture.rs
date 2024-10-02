use crate::config::Config;
use log::{debug, error, warn};
use opencv::{
    core::{self, Size},
    highgui, imgcodecs,
    prelude::*,
    videoio::*,
    Error, Result,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};
use thiserror::Error;

pub mod codec;
pub mod device;
mod file;

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("cannot create file or directory '{0}'")]
    CreateFileDirectory(String),
    #[error("cannot find capture device '{0}'")]
    DeviceNotFound(String),
    #[error("cannot open capture device '{0}'")]
    DeviceOpen(String),
    #[error("cannot grab a frame")]
    GrabFrame,
    #[error("opencv error: {0}")]
    OpenCv(#[from] Error),
    #[error("resource is busy")]
    ResourceBusy,
}

static IS_GRABBING: AtomicBool = AtomicBool::new(false);

pub struct Capture {
    pub config: Config,
    capture: Arc<Mutex<VideoCapture>>,
    writer: Arc<Mutex<Option<VideoWriter>>>,
}

impl Capture {
    pub fn new(conf: &Config) -> Result<Self, CaptureError> {
        debug!("create capture instance");
        let config = conf.clone();
        let device_name = &config.device_name;
        let data_dir = &config.data_dir;
        if file::create_dir(data_dir).is_err() {
            let err = CaptureError::CreateFileDirectory(data_dir.clone());
            error!("{}", err);
            return Err(err);
        }

        let device_id = Self::capture_find_device_by_name(device_name);
        if device_id.is_none() {
            let err = CaptureError::DeviceNotFound(device_name.clone());
            error!("{}", err);
            return Err(err);
        }

        let mut instance = Self::new_capture(device_id.unwrap())?;

        if !instance.is_opened()? {
            let err = CaptureError::DeviceOpen(device_name.clone());
            error!("{}", err);
            return Err(err);
        }

        Self::capture_set_fps(&mut instance, conf.fps)?;
        Self::capture_verify_fps(&instance, conf.fps)?;
        Self::capture_set_frame_size(&mut instance, (conf.frame_width, conf.frame_height))?;
        Self::capture_verify_frame_size(&instance, (conf.frame_width, conf.frame_height))?;

        Ok(Self {
            config,
            capture: Arc::new(Mutex::new(instance)),
            writer: Arc::new(Mutex::new(None)),
        })
    }

    pub fn dispose(&mut self) -> Result<(), CaptureError> {
        debug!("dispose capture instance");
        self.capture
            .lock()
            .unwrap()
            .release()
            .map_err(CaptureError::from)
    }

    pub fn preview(&mut self) -> Result<(), CaptureError> {
        debug!("preview streaming");
        if IS_GRABBING.load(Ordering::Relaxed) {
            let err = CaptureError::ResourceBusy;
            error!("{}", err);
            return Err(err);
        }
        IS_GRABBING.store(true, Ordering::Relaxed);

        let window: &str = &self.config.device_name;
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
        loop {
            let mut frame = Mat::default();
            self.capture.lock().unwrap().read(&mut frame)?;
            if frame.size()?.width > 0 {
                highgui::imshow(window, &frame)?;
            }
            let key = highgui::wait_key(10)?;
            if key == 27 || highgui::get_window_property(window, highgui::WND_PROP_VISIBLE)? < 1.0 {
                break;
            }
        }
        IS_GRABBING.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub fn grab_frame_to_file(&mut self, file_path: &str) -> Result<bool, CaptureError> {
        debug!("grab frame to file '{}'", file_path);
        if IS_GRABBING.load(Ordering::Relaxed) {
            let err = CaptureError::ResourceBusy;
            error!("{}", err);
            return Err(err);
        }
        IS_GRABBING.store(true, Ordering::Relaxed);

        let mut frame = Mat::default();
        let success = self.capture.lock().unwrap().read(&mut frame)?;
        if !success || frame.empty() {
            let err = CaptureError::GrabFrame;
            error!("{}", err);
            return Err(err);
        }
        let mut params = core::Vector::default();
        params.push(imgcodecs::IMWRITE_PNG_COMPRESSION);
        params.push(0);
        imgcodecs::imwrite(file_path, &frame, &params)?;
        IS_GRABBING.store(false, Ordering::Relaxed);
        Ok(success)
    }

    pub fn grab_frame(&mut self) -> Result<bool, CaptureError> {
        let file_path = format!("{}\\{}", &self.config.data_dir, file::get_name("png"));
        self.grab_frame_to_file(&file_path)
    }

    pub fn start_grab_video_to_file(&mut self, file_path: &str) -> Result<bool, CaptureError> {
        debug!("grab video to file '{}'", file_path);
        if IS_GRABBING.load(Ordering::Relaxed) {
            let err = CaptureError::ResourceBusy;
            error!("{}", err);
            return Err(err);
        }
        IS_GRABBING.store(true, Ordering::Relaxed);

        let capture = Arc::clone(&self.capture);
        let writer = Arc::clone(&self.writer);

        let fps = self.get_fps()?;
        let frame_size = self.get_frame_size()?;
        let file_path = file_path.to_string();

        let fourcc = self.config.codec.fourcc()?;
        let mut writer_loc = writer.lock().unwrap();
        *writer_loc = Some(VideoWriter::new(
            &file_path,
            fourcc,
            fps as f64,
            Size::new(frame_size.0 as i32, frame_size.1 as i32),
            true,
        )?);
        drop(writer_loc);

        thread::spawn(move || {
            debug!("spawn grabber thread");

            let start_time = Instant::now();
            let mut frame_count: u64 = 0;

            while IS_GRABBING.load(Ordering::Relaxed) {
                let elapsed = start_time.elapsed();
                let target_frame_count = (elapsed.as_secs_f64() * fps as f64).floor() as u64;

                if frame_count < target_frame_count {
                    let mut frame = Mat::default();
                    if capture.lock().unwrap().read(&mut frame).unwrap() {
                        writer
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .write(&frame)
                            .unwrap();
                    }
                    frame_count += 1;
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }

            let mut writer_lock = writer.lock().unwrap();
            *writer_lock = None;
        });

        Ok(true)
    }

    pub fn start_grab_video(&mut self) -> Result<bool, CaptureError> {
        let file_path = format!(
            "{}\\{}",
            &self.config.data_dir,
            file::get_name(self.config.codec.file_extension())
        );
        self.start_grab_video_to_file(&file_path)
    }

    pub fn stop_grab_video(&mut self) -> Result<(), CaptureError> {
        debug!("stop grabber thread");
        IS_GRABBING.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub fn get_fps(&self) -> Result<u32, CaptureError> {
        Self::capture_get_fps(&self.capture.lock().unwrap()).map_err(CaptureError::from)
    }

    pub fn get_frame_size(&self) -> Result<(u32, u32), CaptureError> {
        Self::capture_get_frame_size(&self.capture.lock().unwrap()).map_err(CaptureError::from)
    }

    pub fn set_fps(&mut self, fps: u32) -> Result<bool, CaptureError> {
        Self::capture_set_fps(&mut (self.capture.lock().unwrap()), fps)
            .map_err(CaptureError::from)?;
        self.config.fps = fps;
        Self::capture_verify_fps(&self.capture.lock().unwrap(), fps).map_err(CaptureError::from)
    }

    pub fn set_frame_size(&mut self, size: (u32, u32)) -> Result<bool, CaptureError> {
        Self::capture_set_frame_size(&mut (self.capture.lock().unwrap()), size)
            .map_err(CaptureError::from)?;
        self.config.frame_width = size.0;
        self.config.frame_height = size.1;
        Self::capture_verify_frame_size(&self.capture.lock().unwrap(), size)
            .map_err(CaptureError::from)
    }

    fn capture_find_device_by_name(name: &str) -> Option<u32> {
        let devices = device::enumerate_capture_devices().ok()?;
        device::get_capture_device_id_by_name(&devices, name)
    }

    fn new_capture(device_id: u32) -> Result<VideoCapture> {
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
