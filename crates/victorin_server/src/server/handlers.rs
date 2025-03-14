use crate::config::config::DevicePin;
use crate::config::config::DeviceRequest;
use crate::config::config::PlantConfig;
use crate::server::server::Tx;
use crate::system::system::System;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use warp::http::StatusCode;

pub async fn activate_device(
    device: DeviceRequest,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    println!("Activating device... {}", device.device_id);

    let mut system = system.lock().await;
    let Some(device) = system
        .devices
        .iter()
        .find(|d| d.id == device.device_id)
        .cloned()
    else {
        return Ok(StatusCode::NOT_FOUND);
    };

    let (tx, rx) = oneshot::channel();

    system.register_cycle_complete_listener(device.id, tx);
    system.register_device(device.clone());

    println!("activated_device {}", device.id);
    drop(system);
    tokio::spawn(async move {
        match tokio::time::timeout(std::time::Duration::from_secs(60), rx).await {
            Ok(Ok(())) => {
                println!("Cycle complete for device {}", device.id);
            }
            Ok(Err(_)) => {
                println!("Cycle complete signal failed for device {}", device.id);
            }
            Err(_) => {
                println!(
                    "Timed out waiting for cycle complete for device {}",
                    device.id
                );
            }
        }
    });

    Ok(StatusCode::ACCEPTED)
}

pub async fn cycle_complete(
    device: DeviceRequest,
    system: Arc<Mutex<System>>,
) -> Result<impl warp::Reply, Infallible> {
    println!("Received cycle complete for device... {}", device.device_id);
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
pub async fn humidity_plant(plant: PlantHumidity, tx: Tx) -> Result<impl warp::Reply, Infallible> {
    let message = format!(r#"{{"id": {}, "humidity": {}}}"#, plant.id, plant.humidity);
    //println!("Broadcasting message: {}", message);

    match tx.send(message.clone()) {
        Ok(_) => {}
        Err(e) => println!("Error sending message: {:?}", e),
    }

    //println!("Subscribers after sending: {}", tx.receiver_count());

    Ok(StatusCode::OK)
}
