// use embedded_hal::adc::OneShot;
// use esp_idf_hal::adc::{AdcChannelDriver, AdcDriver, Atten11dB, Config, Resolution10Bit};
// use esp_idf_hal::gpio::{Gpio2, Gpio32, Gpio33, Output, PinDriver};
// use esp_idf_hal::peripherals::Peripherals;
// use esp_idf_svc::netif::*;
// use esp_idf_svc::nvs::*;
// use esp_idf_svc::wifi::*;
// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::thread;
// use std::time::Duration;
// use std::{io::Read, io::Write, net::TcpListener, net::TcpStream};

// #[derive(Serialize, Deserialize)]
// struct Command {
//     command: String,
//     valve_id: Option<u32>,
//     duration: Option<u64>,
// }

// // Map ADC values to a percentage (0% to 100%)
// fn map_adc_value(value: u16, istart: u16, istop: u16, ostart: u8, ostop: u8) -> u8 {
//     let scaled = ostart as f32
//         + (ostop as f32 - ostart as f32)
//             * ((value as f32 - istart as f32) / (istop as f32 - istart as f32));
//     scaled.clamp(ostart as f32, ostop as f32) as u8
// }

// // HTTP POST function
// fn send_humidity_data(sensor_id: u8, moisture: u8) {
//     let payload = json!({
//         "sensor_id": sensor_id,
//         "humidity": moisture
//     });

//     let client = reqwest::blocking::Client::new();
//     let url = "http://localhost:5000/plant_humidity"; // Update with your actual endpoint

//     match client.post(url).json(&payload).send() {
//         Ok(response) => println!("Sent data to server. Response: {:?}", response.status()),
//         Err(e) => println!("Failed to send data: {:?}", e),
//     }
// }

// // HTTP server handler
// fn handle_request(stream: &mut TcpStream, valve: &mut PinDriver<Gpio2, Output>) {
//     let mut buffer = [0; 1024];
//     let read_size = stream.read(&mut buffer).unwrap_or(0);
//     let request = String::from_utf8_lossy(&buffer[..read_size]);

//     if let Some(body_start) = request.find("\r\n\r\n") {
//         let body = &request[body_start + 4..];

//         if let Ok(command) = serde_json::from_str::<Command>(body) {
//             if command.command == "activate_valve" {
//                 if let (Some(valve_id), Some(duration)) = (command.valve_id, command.duration) {
//                     if valve_id == 2 {
//                         println!("Activating valve {} for {} seconds", valve_id, duration);
//                         valve.set_high().unwrap();
//                         thread::sleep(Duration::from_secs(duration));
//                         valve.set_low().unwrap();
//                         let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Valve activated successfully\"}";
//                         stream.write_all(response.as_bytes()).unwrap();
//                         return;
//                     }
//                 }
//             }
//         }
//     }

//     let error_response = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{\"error\": \"Invalid request\"}";
//     stream.write_all(error_response.as_bytes()).unwrap();
// }

// // Main function
// fn main() -> anyhow::Result<()> {
//     let peripherals = Peripherals::take().unwrap();

//     // Wi-Fi configuration
//     let wifi = EspWifi::new(
//         peripherals.modem,
//         Default::default(),
//         Default::default(),
//         Default::default(),
//     )?;
//     let wifi_config = WifiConfiguration {
//         ssid: "Balais".to_string(),
//         password: "dUst102!".to_string(),
//         ..Default::default()
//     };
//     wifi.set_configuration(&wifi_config)?;
//     wifi.start()?;
//     wifi.connect()?;
//     while !wifi.is_connected().unwrap() {
//         thread::sleep(Duration::from_millis(500));
//     }
//     println!("Connected to Wi-Fi");

//     // ADC initialization
//     let mut adc1 = AdcDriver::new(peripherals.adc1, &Config::new().resolution(Resolution10Bit))?;
//     let mut sensor1 = AdcChannelDriver::<Gpio33, _>::new(peripherals.pins.gpio33)?;
//     let mut sensor2 = AdcChannelDriver::<Gpio32, _>::new(peripherals.pins.gpio32)?;

//     // Valve GPIO initialization
//     let mut valve = PinDriver::output(peripherals.pins.gpio2)?;

//     // Start HTTP server
//     let listener = TcpListener::bind("0.0.0.0:8080")?;
//     println!("Server listening on 0.0.0.0:8080");

//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut stream) => {
//                 println!("Client connected");
//                 handle_request(&mut stream, &mut valve);
//             }
//             Err(e) => println!("Failed to accept connection: {:?}", e),
//         }
//     }

//     Ok(())
// }

use serde_json::json;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let url = format!("http://192.168.0.25:8080/activate"); // Fixed the URL format
    let body = json!({
        "device": "1",
        "duration": 3
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body.to_string()) // Convert the JSON value to a string
        .send()
        .await
        .unwrap(); // Use `?` to propagate errors

    let response_text = response.text().await.unwrap(); // Use `?` to propagate errors
}
