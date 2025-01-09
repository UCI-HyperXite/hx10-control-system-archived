use tracing::info;

use crate::components::yaw::Yaw;

pub async fn read_distance(mut yaw: Yaw){
    info!("Starting Vl53L0x Test.");
    loop {
        let distance = yaw.read_distance(); 
        println!("Distance: {:?}", distance.distance); // log distance value 
        tokio::time::sleep(std::time::Duration::from_millis(50)).await; // waits 50ms before reading again 
    }
}