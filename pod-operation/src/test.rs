use tracing::info;

use crate::components::yaw::Yaw;

pub async fn read_distance(mut yaw: Yaw){
    info!("Starting Vl53L0x Test.");
    loop{
        let distance = yaw.read_distance();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        println!("Distance: {:/}", Distance.distance);
    }
}