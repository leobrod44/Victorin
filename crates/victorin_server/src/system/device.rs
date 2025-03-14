use crate::{config::config::DeviceConfig, plants::plant::Plant};
use chrono::{Duration, Utc};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct Device {
    pub(crate) id: u32,
    pub(crate) ip: String,
    pub(crate) pin: u8,
    pub(crate) duration: Duration,
    pub(crate) status: bool,
    pub(crate) plants: Vec<Plant>,
}

impl From<&DeviceConfig> for Device {
    fn from(config: &DeviceConfig) -> Self {
        let cycle = Duration::seconds(config.cycle_sec);
        let duration = Duration::milliseconds(config.duration_ms);
        let target = config
            .plants
            .iter()
            .map(|plant_config| Plant::from(plant_config))
            .collect();

        Device::new(
            config.device_id,
            config.ip.clone(),
            config.pin,
            config.name.clone(),
            cycle,
            duration,
            target,
        )
    }
}

impl Device {
    pub fn new(
        id: u32,
        ip: String,
        pin: u8,
        name: String,
        cycle: Duration,
        duration: Duration,
        target: Vec<Plant>,
    ) -> Device {
        Device {
            id,
            ip,
            pin,
            duration,
            status: false,
            plants: target,
        }
    }
    pub(crate) async fn activate(&mut self) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("http://{}:8080/activate", self.ip);
        let body = json!({
            "device_gpio": self.pin,
            "duration": self.duration.num_seconds()
        });

        let _ = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?
            .error_for_status()?;

        self.status = true;
        Ok(format!("Device {} activated", self.id))
    }

    pub(crate) fn deactivate(&mut self) {
        self.status = false;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Pump {
    pub(crate) ip: String,
    pub(crate) status: bool,
}

impl Pump {
    pub fn new(pump: Pump) -> Pump {
        Pump {
            ip: pump.ip,
            status: false,
        }
    }
}
