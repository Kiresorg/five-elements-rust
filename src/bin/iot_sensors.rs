use tokio::time::{sleep, Duration};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Shared state to store sensor readings
    let data = Arc::new(Mutex::new(HashMap::new()));

    // Launch async sensor polling tasks
    let sensors = vec!["temp_sensor", "humidity_sensor", "pressure_sensor"];
    for sensor in &sensors {
        let data_clone = Arc::clone(&data);
        let sensor_id = sensor.to_string();

        tokio::spawn(async move {
            poll_sensor(sensor_id, data_clone).await;
        });
    }

    // Main thread updates the dashboard every second
    loop {
        {
            let readings = data.lock().unwrap();
            println!("--- Sensor Dashboard ---");
            for (sensor, value) in readings.iter() {
                println!("{sensor}: {value:.2}");
            }
            println!("-------------------------\n");
        }
        sleep(Duration::from_secs(1)).await;
    }
}

async fn poll_sensor(sensor_name: String, data: Arc<Mutex<HashMap<String, f64>>>) {
    let mut rng = StdRng::from_entropy();

    loop {
        // Simulate delay in fetching sensor data
        sleep(Duration::from_millis(700)).await;

        let reading = rng.gen_range(20.0..100.0); // Simulated value

        let mut readings = data.lock().unwrap();
        readings.insert(sensor_name.clone(), reading);
    }
}

