use icapture_core::capture::Capture;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use warp::Filter;

mod server;
use server::*;

fn main() {
    let state = Arc::new(Mutex::new(None::<Capture>));

    let init = warp::post()
        .and(warp::path("init"))
        .and(warp::body::json())
        .and(with_state(state.clone()))
        .and_then(init_capture);

    let start = warp::post()
        .and(warp::path("start"))
        .and(with_state(state.clone()))
        .and_then(start_grab_video);

    let stop = warp::post()
        .and(warp::path("stop"))
        .and(with_state(state.clone()))
        .and_then(stop_grab_video);

    let routes = init.or(start).or(stop).recover(error::handle_rejection);

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
