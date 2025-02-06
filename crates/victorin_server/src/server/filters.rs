use crate::{server::handlers, system::system::System};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[derive(serde::Deserialize)]
pub struct PlantHumidity {
    pub id: u32,
    pub humidity: f32,
}

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

/// POST activate device
pub fn activate_device(
    system: Arc<Mutex<System>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("activate_device")
        .and(warp::post())
        .and(json_body())
        .and(with_system(system))
        .and_then(handlers::activate_device)
}
/// POST activate device
pub fn cycle_complete(
    system: Arc<Mutex<System>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("cycle_complete")
        .and(warp::post())
        .and(json_body())
        .and(with_system(system))
        .and_then(handlers::activate_device)
}

/// POST notify plant humidity
pub fn humidity_plant(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("humidity_plant")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::humidity_plant)
}

fn with_system(
    system: Arc<Mutex<System>>,
) -> impl Filter<Extract = (Arc<Mutex<System>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&system))
}

fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
