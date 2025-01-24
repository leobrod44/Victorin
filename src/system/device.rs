use crate::{config::config::DeviceConfig, plants::plant::Plant};
use chrono::{DateTime, Duration, Utc};
use rppal::gpio::Gpio;

#[derive(Debug, Clone)]
pub struct Device {
    pub(crate) pin: u8,
    pub(crate) gpio: Gpio,
    pub(crate) name: String,
    pub(crate) cycle: Duration,
    pub(crate) duration: Duration,
    pub(crate) last_trigger: DateTime<Utc>,
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

        Device::new(config.pin, config.name.clone(), cycle, duration, target)
    }
}

impl Device {
    pub fn new(
        pin: u8,
        name: String,
        cycle: Duration,
        duration: Duration,
        target: Vec<Plant>,
    ) -> Device {
        Device {
            pin,
            gpio: Gpio::new().expect("Failed to initialize GPIO"),
            name,
            cycle,
            duration,
            status: false,
            last_trigger: Utc::now(),
            plants: target,
        }
    }
    pub(crate) fn activate(&self) {
        println!("Device activated {}", self.name);
        self.gpio
            .get(self.pin)
            .expect("Failed to get pin")
            .into_output()
            .set_low();
    }
    pub(crate) fn deactivate(&self) {
        println!("Device deactivated {}", self.name);
        self.gpio
            .get(self.pin)
            .expect("Failed to get pin")
            .into_output()
            .set_high();
    }
}

#[derive(Debug, Clone)]
pub struct Pump {
    pub(crate) pin: u8,
    pub(crate) gpio: Gpio,
    pub(crate) status: bool,
}

impl Pump {
    pub fn new(pin: u8) -> Pump {
        Pump {
            pin,
            gpio: Gpio::new().expect("Failed to initialize GPIO"),
            status: false,
        }
    }
    pub(crate) fn activate(&mut self) {
        if !self.status {
            // self.gpio.get(self.pin).expect("Failed to get pin").into_output().set_low();
            self.status = true;
            println!("Pump activated");
        }
    }
    pub(crate) fn deactivate(&mut self) {
        if self.status {
            //self.gpio.get(self.pin).expect("Failed to get pin").into_output().set_high();
            self.status = false;
            println!("Pump deactivated");
        }
    }
}
