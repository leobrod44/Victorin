use chrono::Utc;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tokio::{task, time::sleep};

use crate::config::config::Config;

use super::device::{Device, Pump};

use reqwest::Client;
#[derive(Clone)]
pub struct System {
    pub devices: Vec<Device>,
    pub pump: Pump,
    pub tick: Duration,
    pub open_device_count: u8,
    pub plant_devices: HashMap<i32, Device>,
    to_trigger: Vec<Device>,
}

impl System {
    pub fn init(config: Config) -> System {
        let devices: Vec<Device> = config
            .devices
            .iter()
            .map(|device_config| Device::from(device_config))
            .collect();
        let plant_devices: HashMap<i32, Device> = devices
            .iter()
            .flat_map(|device| {
                device
                    .plants
                    .iter()
                    .map(move |plant| (plant.id, device.clone()))
            })
            .collect();

        System {
            devices: devices,
            pump: Pump::new(config.pump),
            tick: Duration::from_millis(config.tick),
            open_device_count: 0,
            plant_devices,
            to_trigger: vec![],
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

    fn open_device(&self, device: &Device) {
        device.activate();
    }

    fn close_device(&self, device: &Device) {
        device.deactivate();
    }

    fn activate_pump(&mut self) {
        if !self.pump.status {
            self.pump.activate();
        }
    }

    fn deactivate_pump(&mut self) {
        if self.pump.status {
            self.pump.deactivate();
        }
    }

    async fn activate_remote_valve(&self, device: &Device) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("http://{}/{}", device.ip, "activate");
        let body = json!({
            "device": device.pin,
            "status": true
        });
        let response = self.client.post(&url).json(&body).send().await?;
        response.text().await
    }
}

pub async fn check_devices(system: Arc<Mutex<System>>) {
    let now = Utc::now();
    let mut system_guard = system.lock().await;
    let to_notify: Vec<usize> = system_guard
        .devices
        .iter()
        .enumerate()
        .filter_map(|(index, device)| {
            if now.signed_duration_since(device.last_trigger) >= device.cycle && !device.status {
                Some(index)
            } else {
                None
            }
        })
        .collect();

    let to_trigger: Vec<_> = system_guard
        .to_trigger
        .iter()
        .map(|device| Arc::new(Mutex::new(device.clone())))
        .collect();

    if !to_trigger.is_empty() {
        //open devices
        for device in &to_trigger {
            let device_guard = device.lock().await;
            system_guard.open_device(&*device_guard);
        }

        //open pump
        system_guard.activate_pump();

        //start device deactivation timers
        for device in to_trigger {
            println!("Device to trigger: {:?}", device.lock().await.pin);
            let system_clone = Arc::clone(&system);

            task::spawn(async move {
                let duration = {
                    let mut device_guard = device.lock().await;
                    let mut system_guard = system_clone.lock().await;
                    system_guard.deregister_device(&*device_guard);
                    device_guard.status = true;
                    system_guard.open_device_count += 1;
                    Duration::from_millis(device_guard.duration.num_milliseconds() as u64)
                };

                tokio::time::sleep(duration).await;

                {
                    let mut device_guard = device.lock().await;
                    let mut system_guard = system_clone.lock().await;
                    system_guard.open_device_count -= 1;
                    if system_guard.open_device_count == 0 {
                        system_guard.deactivate_pump();
                    }
                    system_guard.close_device(&*device_guard);
                    device_guard.last_trigger = Utc::now();
                    device_guard.status = false;
                }
            });
        }
    }
    async fn request(request_name: String) -> Result<impl warp::Reply, warp::Rejection> {
        let client = Client::new();

        let response = client
            .post("https://jsonplaceholder.typicode.com/posts")
            .json(&json!({
                "title": "foo",
                "body": "bar",
                "userId": 1
            }))
            .send()
            .await;

        match response {
            Ok(resp) => {
                // Get the response body as text
                let body = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to get response".to_string());
                Ok(warp::reply::json(&body))
            }
            Err(err) => {
                eprintln!("HTTP Request failed: {:?}", err);
                Err(warp::reject::custom(err))
            }
        }
    }
}
