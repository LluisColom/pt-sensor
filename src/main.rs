use chrono::{DateTime, Utc};
use rand::Rng;
use serde::Serialize;

const SENSOR_ID: i32 = 1;
const URL: &str = "https://127.0.0.1:3000/sensors/ingest";

#[derive(Debug, Serialize)]
pub struct SensorReading {
    sensor_id: i32,
    timestamp: DateTime<Utc>, // ISO 8601 format
    co2: f32,
    temperature: f32,
}

#[tokio::main]
async fn main() {
    println!("Simulating sensor data!");

    let mut rand = rand::rng();
    let client = reqwest::Client::new();

    loop {
        let co2 = 100000.0 + rand.random_range(-20000.0..30000.0);
        println!("Captured Co2 value: {}", co2);

        let reading = SensorReading {
            sensor_id: SENSOR_ID,
            timestamp: Utc::now(),
            co2,
            temperature: 80.0 + (co2 - 100_000.0).abs() / 2000.0, // rough correlation,
        };

        if let Err(e) = client.post(URL).json(&reading).send().await {
            println!("Error sending reading: {}", e);
        }

        // Sleep between readings
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
