// Function to filter out invalid readings (values below -50°C or above 60°C)
fn filter_invalid_readings(readings: Vec<f64>) -> Vec<f64> {
    let mut valid_readings = Vec::new();
    for reading in readings {
        if reading >= -50.0 && reading <= 60.0 {
            valid_readings.push(reading);
        }
    }
    valid_readings
    /*readings
    .into_iter()
    .filter(|&reading| reading >= -50.0 && reading <= 60.0)
    .collect()*/
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// Function to calculate the average of a vector of f64
fn calculate_average(readings: &[f64]) -> f64 {
    let sum: f64 = readings.iter().sum();
    sum / readings.len() as f64
}

// Function to process sensor data and return the sensor with the highest average temperature
fn process_sensor_data(sensor_data: &[(String, Vec<f64>)]) -> (String, f64) {
    let mut max_avg = f64::MIN;
    let mut max_sensor = String::new();

    for (sensor_name, readings) in sensor_data {
        let valid_readings = filter_invalid_readings(readings.clone());
        let fahrenheit_readings: Vec<f64> = valid_readings
            .iter()
            .map(|&c| celsius_to_fahrenheit(c))
            .collect();
        let avg_temp = calculate_average(&fahrenheit_readings);

        println!(
            "Sensor {}: Average temperature = {:.2}°F",
            sensor_name, avg_temp
        );

        if avg_temp > max_avg {
            max_avg = avg_temp;
            max_sensor = sensor_name.clone();
        }
    }

    (max_sensor, max_avg)
}

pub fn exercise_3() {
    let sensor_data: Vec<(String, Vec<f64>)> = vec![
        ("sensor_1".to_string(), vec![22.5, 23.0, 22.8, -60.0, 23.3]),
        ("sensor_2".to_string(), vec![18.0, 19.5, 18.7, 20.0, 19.2]),
        ("sensor_3".to_string(), vec![25.0, 24.8, 25.2, 25.1, 24.9]),
    ];

    let (max_sensor, _) = process_sensor_data(&sensor_data);
    println!(
        "Sensor with the highest average temperature: {}",
        max_sensor
    );
}
