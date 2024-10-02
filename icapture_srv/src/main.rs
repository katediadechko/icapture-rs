use icapture_core::{capture::Capture, config::Config};
use log::warn;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use warp::{Filter, Rejection};

mod server;
use server::*;

fn main() {
    env_logger::builder().format_timestamp_millis().init();
    let log = warp::log("icapture_srv::api");

    let state = Arc::new(Mutex::new(None::<Capture>));

    let list = warp::get()
        .and(warp::path("list"))
        .and_then(list_devices);

    let init = warp::post()
        .and(warp::path("init"))
        .and(warp::body::json().or_else(|_| async {
            warn!("cannot parse request body, falling back to default config");
            Ok::<(Config,), Rejection>((Config::default(),))
        }))
        .and(with_state(state.clone()))
        .and_then(init_capture);

    let grab = warp::post()
        .and(warp::path("frame"))
        .and(with_state(state.clone()))
        .and_then(grab_frame);

    let start = warp::post()
        .and(warp::path("start"))
        .and(with_state(state.clone()))
        .and_then(start_grab_video);

    let stop = warp::post()
        .and(warp::path("stop"))
        .and(with_state(state.clone()))
        .and_then(stop_grab_video);

    let dispose = warp::post()
        .and(warp::path("deinit"))
        .and(with_state(state.clone()))
        .and_then(dispose_capture);

    let routes = list
        .or(init)
        .or(grab)
        .or(start)
        .or(stop)
        .or(dispose)
        .recover(error::handle_rejection)
        .with(log);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        warp::serve(routes).run(([127, 0, 0, 1], 1212)).await;
    });
}

fn with_state(
    state: CaptureState,
) -> impl Filter<Extract = (CaptureState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
