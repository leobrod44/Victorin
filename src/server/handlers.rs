use crate::system::system::System;
use crate::{config::config::PlantConfig, plants::plant::Plant};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub async fn create_plant(
    plant: PlantConfig,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    print!("Creating plant: {:?}", plant);
    Ok(StatusCode::CREATED)
}

pub async fn water_plant(
    plant: PlantConfig,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut system = system.lock().unwrap();
    let Some(device) = system.plant_devices.get(&plant.id).cloned() else {
        return Ok(StatusCode::NOT_FOUND);
    };
    system.register_device(device);
    Ok(StatusCode::OK)
}
