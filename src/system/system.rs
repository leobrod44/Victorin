use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use chrono::Utc;
use tokio::{task, time::sleep};

use crate::config::config::Config;

use super::device::{Device, Pump};

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

    pub async fn run(&self) {
        let start = Utc::now();
        let tick = self.tick;
        let system: Arc<Mutex<System>> = Arc::new(Mutex::new(self.clone()));
        loop {
            check_devices(&system);
            let _ = sleep(tick).await;
        }
    }

    pub fn register_device(&mut self, device: Device) {
        self.devices.push(device);
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
}

pub fn check_devices(system: &Arc<Mutex<System>>) {
    let now = Utc::now();

    let to_trigger: Vec<usize> = {
        let system = system.lock().unwrap();
        system
            .devices
            .iter()
            .enumerate()
            .filter_map(|(index, device)| {
                if now.signed_duration_since(device.last_trigger) >= device.cycle && !device.status
                {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    };

    if !to_trigger.is_empty() {
        {
            let mut system = system.lock().unwrap();
            for &index in &to_trigger {
                system.open_device(&system.devices[index]);
            }
            system.activate_pump();
        }

        // Start device deactivation timers
        for index in to_trigger {
            let system_ref: Arc<Mutex<System>> = Arc::clone(&system);
            task::spawn(async move {
                {
                    let mut system = system_ref.lock().unwrap();
                    system.open_device_count += 1;
                    system.devices[index].status = true;
                }

                let duration_secs = {
                    let system = system_ref.lock().unwrap();
                    system.devices[index].duration.num_milliseconds() as u64
                };
                tokio::time::sleep(Duration::from_millis(duration_secs)).await;
                {
                    let mut system = system_ref.lock().unwrap();
                    system.open_device_count -= 1;
                    if system.open_device_count == 0 {
                        system.deactivate_pump();
                        println!("")
                    }
                    system.close_device(&system.devices[index]);
                    system.devices[index].last_trigger = Utc::now();
                    system.devices[index].status = false;
                }
            });
        }
    }
}
