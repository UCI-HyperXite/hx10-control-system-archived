use tracing::info;

use crate::components::yaw::Yaw;

pub async fn read_distance(mut yaw: Yaw) {
    info!("Starting VL53L0X Test.");
    loop {
        match yaw.read_distance() {
            Ok(distance) => println!("Distance: {} mm", distance.distance),
            Err(e) => eprintln!("Failed to read distance: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_millis(300)).await; // Match delay with period
    }
}