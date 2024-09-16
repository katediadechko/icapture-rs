use icapture_core::{capture::*, config::*};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use warp::{reject::Rejection, Reply};

pub mod error;
use error::*;

type Result<T> = std::result::Result<T, Rejection>;
pub(crate) type CaptureState = Arc<Mutex<Option<Capture>>>;

#[derive(Deserialize)]
pub(crate) struct ConfigPath {
    path: String,
}

pub(crate) async fn init_capture(config: ConfigPath, state: CaptureState) -> Result<impl Reply> {
    let config = Config::new(&config.path);
    let capture = Capture::new(&config.clone())
        .map_err(|e| warp::reject::custom(ApiError::CaptureError(e)))?;

    let mut state = state.lock().unwrap();
    *state = Some(capture);

    Ok(warp::reply::json(&error::StatusResponse {
        message: format!("Capture initialized successfully: {:?}", &config),
    }))
}

pub(crate) async fn grab_frame(state: CaptureState) -> Result<impl Reply> {
    let mut state = state.lock().unwrap();
    if let Some(capture) = state.as_mut() {
        capture
            .grab_frame()
            .map_err(|e| warp::reject::custom(ApiError::CaptureError(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "Frame grabbed".to_string(),
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
            .map_err(|e| warp::reject::custom(ApiError::CaptureError(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "Video grab started".to_string(),
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
            .map_err(|e| warp::reject::custom(ApiError::CaptureError(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "Video grab stopped".to_string(),
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
            .map_err(|e| warp::reject::custom(ApiError::CaptureError(e)))?;

        Ok(warp::reply::json(&StatusResponse {
            message: "Capture disposed".to_string(),
        }))
    } else {
        Err(warp::reject::custom(ApiError::CaptureNotInitialized))
    }
}
