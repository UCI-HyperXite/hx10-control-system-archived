use tracing::info;

// use crate::components::yaw::Yaw;
use crate::components::front_tof::*;
use crate::components::center_tof::*;

pub async fn read_front_tof(mut front_tof: Front_Tof) {
    info!("Starting VL53L0X-1 Test.");
    loop {
        match front_tof.read_distance() {
            Ok(distance) => println!("Front Sensor: {}", distance.distance),
            Err(e) => eprintln!("Failed to read yaw: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await; // Match delay with period
    }
}

pub async fn read_center_tof(mut center_tof: Center_Tof) {
    info!("Starting VL53L0X-2 Test.");
    loop {
        match center_tof.read_distance() {
            Ok(distance) => println!("Center Sensor: {}", distance.distance),
            Err(e) => eprintln!("Failed to read yaw: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await; // Match delay with period
    }
}