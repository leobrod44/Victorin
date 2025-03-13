use crate::config::config::DeviceConfig;
use crate::config::config::PlantConfig;
use crate::system::system::System;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use warp::http::StatusCode;

pub async fn activate_device(
    device: DeviceConfig,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut system = system.lock().await;
    let Some(device) = system.plant_devices.get(&device.device_id).cloned() else {
        return Ok(StatusCode::NOT_FOUND);
    };
    let (tx, rx) = oneshot::channel();

    system.register_cycle_complete_listener(device.id, tx);
    let _ = system.activate_remote_valve(&device).await;

    println!("activated_device {}", device.id);

    match tokio::time::timeout(std::time::Duration::from_secs(60), rx).await {
        Ok(Ok(())) => {
            println!("Cycle complete for device {}", device.id);
            Ok(StatusCode::OK)
        }
        Ok(Err(_)) => {
            println!("Cycle complete signal failed for device {}", device.id);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(_) => {
            println!(
                "Timed out waiting for cycle complete for device {}",
                device.id
            );
            Ok(StatusCode::REQUEST_TIMEOUT)
        }
    }
}

pub async fn cycle_complete(
    device: DeviceConfig,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut system = system.lock().await;
    system.complete_cycle(device.device_id);
    Ok(StatusCode::OK)
}

use super::filters::PlantHumidity;

pub async fn create_plant(
    plant: PlantConfig,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    println!("Creating plant...");

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
pub async fn humidity_plant(plant: PlantHumidity) -> Result<impl warp::Reply, Infallible> {
    println!("plant humidity: {}, {}", plant.id, plant.humidity);
    //TODO send plant data to app db
    Ok(StatusCode::OK)
}
