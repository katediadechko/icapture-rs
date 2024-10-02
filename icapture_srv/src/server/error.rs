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
    Capture(#[from] CaptureError),
    #[error("capture not initialized")]
    CaptureNotInitialized,
    #[error("cannot enumerate capture devices")]
    EnumerateDevices,
}
impl warp::reject::Reject for ApiError {}

pub(crate) async fn handle_rejection(
    err: Rejection,
) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if let Some(e) = err.find::<ApiError>() {
        match e {
            ApiError::Capture(CaptureError::CreateFileDirectory(path)) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot create file or directory '{}'", path),
            ),
            ApiError::Capture(CaptureError::DeviceOpen(device)) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("cannot open capture device {}", device),
            ),
            ApiError::Capture(CaptureError::GrabFrame) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "cannot grab a frame".to_string(),
            ),
            ApiError::Capture(CaptureError::OpenCv(error)) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("opencv error: {}", error),
            ),
            ApiError::Capture(CaptureError::ResourceBusy) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "resource is busy".to_string(),
            ),
            ApiError::CaptureNotInitialized => (
                warp::http::StatusCode::BAD_REQUEST,
                "capture not initialized".to_string(),
            ),
            ApiError::EnumerateDevices => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "cannot enumerate capture devices".to_string(),
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
