use icapture_core::capture::CaptureError;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{reject::Rejection, Reply};

#[derive(Serialize)]
pub(crate) struct StatusResponse {
    pub(crate) message: String,
}

#[derive(Error, Debug)]
pub(crate) enum ApiError {
    #[error("capture error: {0}")]
    CaptureError(#[from] CaptureError),
    #[error("capture not initialized")]
    CaptureNotInitialized,
}
impl warp::reject::Reject for ApiError {}

pub(crate) async fn handle_rejection(
    err: Rejection,
) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if let Some(e) = err.find::<ApiError>() {
        match e {
            ApiError::CaptureError(CaptureError::CreateFileDirectory(path)) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot create file or directory '{}'", path),
            ),
            ApiError::CaptureError(CaptureError::DeviceNotFound(device)) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot find capture device '{}'", device),
            ),
            ApiError::CaptureError(CaptureError::DeviceOpenError(device)) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot find capture device '{}'", device),
            ),
            ApiError::CaptureError(CaptureError::FrameError) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "cannot grab a frame".to_string(),
            ),
            ApiError::CaptureError(CaptureError::OpenCvError(error)) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("opencv error: {}", error),
            ),
            ApiError::CaptureError(CaptureError::ResourceBusyError) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "resource is busy".to_string(),
            ),
            ApiError::CaptureNotInitialized => (
                warp::http::StatusCode::BAD_REQUEST,
                "capture not initialized".to_string(),
            ),
        }
    } else {
        (
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            "unhandled error".to_string(),
        )
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&StatusResponse { message }),
        code,
    ))
}
