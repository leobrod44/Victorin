use chrono::Utc;
use serde_json::json;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{oneshot, Mutex};
use tokio::{task, time::sleep};

use crate::config::config::Config;
use reqwest::Error;

use super::device::{Device, Pump};

pub struct System {
    pub devices: Vec<Device>,
    pub pump: Pump,
    pub tick: Duration,
    pub open_device_count: u8,
    pub plant_devices: HashMap<u32, Device>,
    to_trigger: Vec<Device>,
    cycle_listeners: HashMap<u32, oneshot::Sender<()>>,
}

impl System {
    pub fn init(config: Config) -> System {
        let devices: Vec<Device> = config
            .devices
            .iter()
            .map(|device_config| Device::from(device_config))
            .collect();
        let plant_devices: HashMap<u32, Device> = devices
            .iter()
            .flat_map(|device| {
                device
                    .plants
                    .iter()
                    .map(move |plant| (plant.valve_id, device.clone()))
            })
            .collect();

        System {
            devices: devices,
            pump: Pump::new(config.pump),
            tick: Duration::from_millis(config.tick),
            open_device_count: 0,
            plant_devices,
            to_trigger: vec![],
            cycle_listeners: HashMap::new(),
        }
    }

    pub async fn run(system: Arc<Mutex<Self>>) {
        let system_guard = system.lock().await;
        let tick = system_guard.tick;
        drop(system_guard);

        loop {
            check_devices(Arc::clone(&system)).await;
            let _ = sleep(tick).await;
        }
    }

    pub fn register_device(&mut self, device: Device) {
        self.to_trigger.push(device);
    }
    pub fn deregister_device(&mut self, device: &Device) {
        self.to_trigger.retain(|d| d.pin != device.pin);
    }

    async fn open_device(&mut self, device: &mut Device) -> Result<String, reqwest::Error> {
        device.activate().await
    }

    async fn close_device(&mut self, device: &mut Device) {
        device.deactivate();
    }

    async fn activate_pump(&mut self) -> Result<String, reqwest::Error> {
        if !self.pump.status {
            let client = reqwest::Client::new();
            let url = format!("http://{}:8080/activate_pump", self.pump.ip); // Updated URL for pump activation
            let body = json!({
                "device_gpio": self.pump.pin
            });

            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(body.to_string())
                .send()
                .await?
                .error_for_status()?;

            let response_text = response.text().await?;
            self.pump.status = true;
            Ok(response_text)
        } else {
            Ok("Pump is already active".to_string())
        }
    }

    async fn deactivate_pump(&mut self) -> Result<String, reqwest::Error> {
        if self.pump.status {
            let client = reqwest::Client::new();
            let url = format!("http://{}:8080/deactivate_pump", self.pump.ip); // Updated URL for pump activation
            let body = json!({
                "device_gpio": self.pump.pin
            });

            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(body.to_string())
                .send()
                .await?
                .error_for_status()?;

            let response_text = response.text().await?;
            self.pump.status = false;
            Ok(response_text)
        } else {
            Ok("Pump is already inactive".to_string())
        }
    }

    pub async fn activate_remote_valve(&self, device: &Device) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let url = format!("http://{}:8080/activate", device.ip); // Fixed the URL format
        let body = json!({
            "device": device.pin,
            "duration": device.cycle.num_seconds()
        });

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string()) // Convert the JSON value to a string
            .send()
            .await?; // Use `?` to propagate errors

        let response_text = response.text().await?; // Use `?` to propagate errors

        Ok(response_text)
    }

    pub fn register_cycle_complete_listener(
        &mut self,
        device_id: u32,
        sender: oneshot::Sender<()>,
    ) {
        self.cycle_listeners.insert(device_id, sender);
    }

    pub fn complete_cycle(&mut self, device_id: u32) {
        if let Some(sender) = self.cycle_listeners.remove(&device_id) {
            let _ = sender.send(());
        }
    }
}

pub async fn check_devices(system: Arc<Mutex<System>>) -> Result<(), Error> {
    let system_guard = system.lock().await;
    let to_trigger: Vec<_> = system_guard
        .to_trigger
        .iter()
        .map(|device| Arc::new(Mutex::new(device.clone())))
        .collect();

    drop(system_guard);
    if !to_trigger.is_empty() {
        // Open devices
        for device in &to_trigger {
            let mut device_guard = device.lock().await;
            let mut system_guard = system.lock().await;
            system_guard.open_device(&mut *device_guard).await?;
        }

        //open pump
        {
            let mut system_guard = system.lock().await;
            match system_guard.activate_pump().await {
                Ok(_) => println!("Pump activated"),
                Err(e) => println!("Error activating pump: {:?}", e),
            };
        }
        let system_clone = Arc::clone(&system);
        task::spawn(async move {
            let mut system_guard = system_clone.lock().await;
            while system_guard.open_device_count > 0 {
                sleep(system_guard.tick).await;
                println!("Open device count: {}", system_guard.open_device_count);
            }
            match system_guard.deactivate_pump().await {
                Ok(_) => println!("Pump deactivated"),
                Err(e) => println!("Error deactivating pump: {:?}", e),
            };
        });
    }
    Ok(())
}
