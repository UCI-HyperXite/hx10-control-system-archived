use tracing::info;

use crate::components::yaw::Yaw;

pub async fn read_distance(mut yaw: Yaw) {
    info!("Starting VL53L0X Test.");
    loop {
        match yaw.read_yaw() {
            Ok(yaw) => println!("Yaw: {}", distance.yaw),
            Err(e) => eprintln!("Failed to read yaw: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await; // Match delay with period
    }
}