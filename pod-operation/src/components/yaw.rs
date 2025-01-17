use {
    vl53l0x::VL53L0x,
    rppal::i2c::I2c,
    components::front_tof::Front_Tof,
    components::center_tof::Center_Tof,
    tokio::task::JoinHandle,
};


pub struct Yaw {
    front_tof: Front_Tof,
    center_tof: Center_Tof,
}

pub struct Angle {
    pub yaw: f64,
}

impl Yaw {
    /// Initialize the sensor for continuous reading
    pub fn new(front_tof: Front_Tof, center_tof: Center_Tof) -> Self {
    
        Yaw { front_tof, center_tof }
    }    
    
    /// Read current distance in continuous mode
    pub fn read_yaw(&mut self) -> Result<Angle, String> {
        print!("Calculating yaw");

        let sensor_task_front: JoinHandle<Result<Distance, String>> = tokio::spawn(async { self.front_tof.read_distance() });
        let sensor_task_center: JoinHandle<Result<Distance, String>> = tokio::spawn(async { self.center_tof.read_distance() });

        let front_distance = sensor_task_front.await.unwrap().unwrap().distance;
        let center_distance = sensor_task_center.await.unwrap().unwrap().distance;



        let yaw: f64 = (((center_distance as f64) - (front_distance as f64)) / 20.0).atan();

        Ok (Angle {yaw})
        
    }      
}