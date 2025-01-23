use chrono::{DateTime, Duration, Utc};
use rppal::gpio::Gpio;

use crate::plants::plant::Plant;

#[derive(Debug, Clone)]
pub struct Device {
    pub(crate) pin: u8,
    pub(crate) gpio: Gpio,
    pub(crate) name: String,
    pub(crate) cycle: Duration,
    pub(crate) duration: Duration,
    pub(crate) last_trigger: DateTime<Utc>,
    pub(crate) status: bool,
    pub(crate) target: Vec<Plant>,
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
            target,
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
