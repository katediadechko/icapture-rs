use icapture_core::{device, Capture, Config};
use std::sync::{Arc, Mutex};
use warp::{reject::Rejection, Reply};

pub mod error;
use error::*;

type Result<T> = std::result::Result<T, Rejection>;
pub(crate) type CaptureState = Arc<Mutex<Option<Capture>>>;

pub(crate) async fn list_devices() -> Result<impl Reply> {
    let list = device::enumerate_capture_devices().ok();
    match list {
        Some(list) => Ok(warp::reply::json(&list)),
        None => Err(warp::reject::custom(ApiError::EnumerateDevices))
    }
}

pub(crate) async fn init_capture(config: Config, state: CaptureState) -> Result<impl Reply> {
    let capture = Capture::new(&config)
        .map_err(|e| warp::reject::custom(ApiError::Capture(e)))?;

    let mut state = state.lock().unwrap();
    *state = Some(capture);

    Ok(warp::reply::json(&error::StatusResponse {
        message: format!("capture initialized successfully: {:?}", &config),
    }))
}

pub(crate) async fn preview(state: CaptureState) -> Result<impl Reply> {
    let mut state = state.lock().unwrap();
    if let Some(capture) = state.as_mut() {
        capture
            .preview()
            .map_err(|e| warp::reject::custom(ApiError::Capture(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "preview executed".to_string(),
        }))
    } else {
        Err(warp::reject::custom(ApiError::CaptureNotInitialized))
    }
}

pub(crate) async fn grab_frame(state: CaptureState) -> Result<impl Reply> {
    let mut state = state.lock().unwrap();
    if let Some(capture) = state.as_mut() {
        capture
            .grab_frame()
            .map_err(|e| warp::reject::custom(ApiError::Capture(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "frame grabbed".to_string(),
        }))
    } else {
        Err(warp::reject::custom(ApiError::CaptureNotInitialized))
    }
}

pub(crate) async fn start_grab_video(state: CaptureState) -> Result<impl Reply> {
    let mut state = state.lock().unwrap();
    if let Some(capture) = state.as_mut() {
        capture
            .start_grab_video()
            .map_err(|e| warp::reject::custom(ApiError::Capture(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "video grab started".to_string(),
        }))
    } else {
        Err(warp::reject::custom(ApiError::CaptureNotInitialized))
    }
}

pub(crate) async fn stop_grab_video(state: CaptureState) -> Result<impl Reply> {
    let mut state = state.lock().unwrap();
    if let Some(capture) = state.as_mut() {
        capture
            .stop_grab_video()
            .map_err(|e| warp::reject::custom(ApiError::Capture(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "video grab stopped".to_string(),
        }))
    } else {
        Err(warp::reject::custom(ApiError::CaptureNotInitialized))
    }
}

pub(crate) async fn dispose_capture(state: CaptureState) -> Result<impl Reply> {
    let mut state = state.lock().unwrap();
    if let Some(mut capture) = state.take() {
        capture
            .dispose()
            .map_err(|e| warp::reject::custom(ApiError::Capture(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "capture disposed".to_string(),
        }))
    } else {
        Err(warp::reject::custom(ApiError::CaptureNotInitialized))
    }
}
