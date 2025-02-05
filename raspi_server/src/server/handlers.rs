use crate::config::config::PlantConfig;
use crate::system::system::System;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

use super::filters::PlantHumidity;

pub async fn create_plant(
    plant: PlantConfig,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    println!("Creating plant...");
    let mut system = system.lock().await;
    println!("Lock acquired...");
    let Some(device) = system.plant_devices.get(&plant.id).cloned() else {
        return Ok(StatusCode::NOT_FOUND);
    };
    system.register_device(device);
    Ok(StatusCode::CREATED)
}

pub async fn water_plant(
    plant: PlantConfig,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut system = system.lock().await;
    let Some(device) = system.plant_devices.get(&plant.id).cloned() else {
        return Ok(StatusCode::NOT_FOUND);
    };
    println!("device: {:?}", device.pin);
    system.register_device(device);
    Ok(StatusCode::OK)
}

pub async fn notify_humidity_plant(plant: PlantHumidity) -> Result<impl warp::Reply, Infallible> {
    println!("plant humidity: {}, {}", plant.id, plant.humidity);
    
    Ok(StatusCode::OK)
}
