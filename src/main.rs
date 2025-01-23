use chrono::{DateTime, Duration, Utc};
use rppal::gpio::Gpio;
use std::thread::sleep;
use victorin::plants::plant::Plant;
use victorin::system::device::{Device, Pump};
use victorin::system::system::System;

#[tokio::main]
async fn main() {
    let system = System::init(
        vec![
            Device::new(
                14,
                "Valve 1".to_string(),
                Duration::seconds(2),
                Duration::milliseconds(1000),
                vec![Plant::new(1, "Plant 1".to_string())],
            ),
            Device::new(
                15,
                "Valve 2".to_string(),
                Duration::seconds(10),
                Duration::milliseconds(8000),
                vec![Plant::new(3, "Plant 2".to_string())],
            ),
        ],
        Pump::new(17),
        std::time::Duration::from_millis(250),
    );
    system.run().await;
}
