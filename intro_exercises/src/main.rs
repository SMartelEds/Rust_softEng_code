fn main() {
    let sensor_data: Vec<(&str, Vec<f64>)> = vec![
        ("sensor_1", vec![22.5, 23.0, 22.8, -60.0, 23.3]),
        ("sensor_2", vec![18.0, 19.5, 18.7, 20.0, 19.2]),
        ("sensor_3", vec![25.0, 24.8, 25.2, 25.1, 24.9]),
    ];

    // println!("{}", sensor_data[0].1[0])
    filter_invalid_readings(sensor_data[0].1);
}

fn filter_invalid_readings(data: Vec<f64>) {
    let mut valids: Vec<f64> = Vec::new();
    for val in data {
        if val >= -50.0 && val <= 60.0 {
            valids.push(val)
        }
    }

    for n in valids {
        println!("elt : {}", n)
    }
}
/*
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
*/
