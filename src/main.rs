use rppal::gpio::Gpio;
use std::thread::sleep;
use crate::plant::Plant;
use chrono::{DateTime, Duration, Utc};

pub mod plant;

struct Device {
    pin: u8,
    gpio: Gpio,
    name: String,
    cycle: Duration,
    duration: Duration,
    last_trigger: DateTime<Utc>,
    target : Vec<Plant>
}

impl Device {
    fn new(pin: u8, name: String, cycle: Duration, duration: Duration, target: Vec<Plant>) -> Device {
        Device {
            pin,
            gpio: Gpio::new().expect("Failed to initialize GPIO"),
            name,
            cycle,
            duration
            last_trigger: Utc::now(),
            target,
        }
    }
    fn activate(&mut self){
        self.gpio.get(self.pin).expect("Failed to get pin").into_output().set_high();
        self.last_trigger = Utc::now();
    }
    fn deactivate(&self) {
        self.gpio.get(self.pin).expect("Failed to get pin").into_output().set_low();
    }
}

struct Pump{
    pin: u8,
    gpio: Gpio,
    status: bool,
}

impl Pump {
    fn new(pin: u8) -> Pump {
        Pump {
            pin,
            gpio: Gpio::new().expect("Failed to initialize GPIO"),
            status: false,
        }
    }
    fn activate(&self) {
        self.gpio.get(self.pin).expect("Failed to get pin").into_output().set_high();
    }
    fn deactivate(&self) {
        self.gpio.get(self.pin).expect("Failed to get pin").into_output().set_low();
    }
}

struct System {
    devices: Vec<Device>,
    open: bool,
}

impl System {

    fn init(devices: Vec<Device>) -> System {
        System {
            devices,
            open: false,
        }
    }

    fn check_and_activate(&mut self) {
        let now = Utc::now();
        for device in &mut self.devices {
            if now.signed_duration_since(device.last_trigger) >= device.cycle {
                println!("{} activated", device.name);
                device.activate();
                sleep( std::time::Duration::from_secs(device.duration.num_seconds() as u64));
                device.deactivate();
            }
        }
    }

    fn activate(&mut self) {
        loop {
            self.check_and_activate();
            sleep(std::time::Duration::from_secs(1)); 
        }
    }
}

fn main() {
    let mut system = System::init(vec![
        Device::new(14, "Valve 1".to_string(),  Duration::seconds(2), Duration::seconds(1), vec![Plant::new(1, "Plant 1".to_string())]),
        Device::new(17, "Valve 2".to_string(), Duration::seconds(3),Duration::seconds(1), vec![Plant::new(2, "Plant 2".to_string())]),
        Device::new(18, "Pump".to_string(), Duration::seconds(7),Duration::seconds(1), vec![Plant::new(3, "Plant 3".to_string())]),
    ]);
    system.activate();
}

