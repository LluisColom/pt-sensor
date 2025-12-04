# Pollution Tracker -  Sensor Simulator

A Rust-based IoT sensor simulator that generates realistic pollution readings (CO2 levels and temperature) and sends them to the Pollution Tracker backend via HTTPS.

## Overview

This simulator mimics a physical pollution monitoring sensor deployed at an industrial site. It generates synthetic sensor data with realistic variations and transmits readings to the backend every 10 seconds for blockchain anchoring and storage.

## Features

- **Realistic Data Generation**: CO2 levels fluctuate within industrial ranges with random variations
- **Temperature Correlation**: Temperature values correlate with CO2 levels to simulate realistic conditions
- **Continuous Monitoring**: Automatic data transmission at configurable intervals
- **HTTP/HTTPS Support**: Can communicate with both secure and non-secure endpoints
- **Error Handling**: Graceful error reporting for network failures

## Prerequisites

- Rust 1.70 or higher
- Access to Pollution Tracker backend (running on localhost:3000 or remote server)

## Installation

1. Clone the repository (or navigate to the sensor directory):
```bash
cd pt-sensor
```

2. Install dependencies:
```bash
cargo build
```

## Configuration

### Basic Configuration

Edit the constants in `src/main.rs`:
```rust
const SENSOR_ID: i32 = 1;                              // Your sensor's unique ID
const URL: &str = "https://127.0.0.1:3000/sensors/ingest";  // Backend endpoint
```

## Usage

### Running with HTTPS (Production)
```bash
cargo run --release
```
## Data Format

The simulator sends JSON payloads in the following format:
```json
{
  "sensor_id": 1,
  "timestamp": "2024-12-04T10:30:45.123Z",
  "co2": 95000.0,
  "temperature": 82.5
}
```

**Field Descriptions:**
- `sensor_id`: Unique identifier for this sensor (configured via `SENSOR_ID`)
- `timestamp`: ISO 8601 formatted timestamp (automatically generated)
- `co2`: CO2 concentration in ppm (parts per million)
- `temperature`: Temperature in Celsius

## Simulation Logic

### CO2 Generation
- **Base value**: 100,000 ppm (industrial emissions baseline)
- **Variation**: ±20,000 to +30,000 ppm random fluctuation
- **Range**: 80,000 - 130,000 ppm

### Temperature Correlation
- **Base temperature**: 80°C
- **Correlation factor**: Temperature increases with CO2 deviation from baseline
- **Formula**: `temp = 80.0 + |co2 - 100000| / 2000`
- **Range**: ~80-95°C

### Transmission Interval
- **Default**: Every 10 seconds
- **Configurable**: Modify `tokio::time::Duration::from_secs(10)` in code

## Integration with Pollution Tracker

This sensor integrates with the Pollution Tracker backend at the `/sensors/ingest` endpoint. The backend will:

1. Validate the incoming sensor reading
2. Generate a cryptographic hash (SHA-256) of the data
3. Submit the hash to Solana blockchain
4. Store the reading and blockchain transaction ID in PostgreSQL

## Customization

### Adjusting CO2 Range
```rust
// More extreme variations (industrial accident simulation)
let co2 = 100000.0 + rand.random_range(-50000.0..80000.0);

// Stable readings (normal operations)
let co2 = 100000.0 + rand.random_range(-5000.0..5000.0);
```

### Changing Transmission Frequency
```rust
// Every 5 seconds (high-frequency monitoring)
tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

// Every minute (low-frequency monitoring)
tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
```

## License

This sensor simulator is part of the Pollution Tracker project developed for the MSc in Cybersecurity program at UPC Barcelona.

## Author

Lluís Colom - MSc Cybersecurity, UPC Barcelona

## Contributing

This is an academic project. For questions or improvements, please contact the author.
