use serde::{Deserialize, Serialize};
use std::fs;

use crate::system::device::Pump;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub devices: Vec<DeviceConfig>,
    pub pump: Pump,
    pub tick: u64,
}

#[derive(Deserialize, Debug)]
pub struct DeviceConfig {
    pub device_id: u32,
    pub ip: String,
    pub name: String,
    pub pin: u8,
    pub cycle_sec: i64,
    pub duration_ms: i64,
    pub plants: Vec<PlantConfig>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceRequest {
    pub device_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct DevicePin {
    pub device_gpio: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlantConfig {
    pub id: u32,
    pub name: String,
}

impl Config {
    pub fn init(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(config_path)?;
        let config: Config = serde_yaml::from_str(&config_str)?;
        Ok(config)
    }
}
