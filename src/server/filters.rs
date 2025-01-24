use crate::{config::config::PlantConfig, server::handlers, system::system::System};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{filters::method::post, Filter};

/// POST new plant
pub fn create_plant(
    system: Arc<Mutex<System>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("create_plant")
        .and(warp::post())
        .and(json_body())
        .and(with_system(system))
        .and_then(handlers::create_plant)
}

/// POST water plant
pub fn water_plant(
    system: Arc<Mutex<System>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("water_plant")
        .and(warp::post())
        .and(json_body())
        .and(with_system(system))
        .and_then(handlers::water_plant)
}

fn with_system(
    system: Arc<Mutex<System>>,
) -> impl Filter<Extract = (Arc<Mutex<System>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&system))
}

fn json_body() -> impl Filter<Extract = (PlantConfig,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
