use {
    vl53l0x::VL53L0x,
    rppal::i2c::I2c,
};


pub struct Yaw {
    front_tof: VL53L0x<I2c>,
    center_tof: VL53L0x<I2c>,
}

pub struct Distance {
    pub yaw: f64,
}

impl Yaw {
    /// Initialize the sensor for continuous reading
    pub fn new() -> Self {
        let front_i2c: I2c = I2c::new().unwrap();
        let center_i2c: I2c = I2c::new().unwrap();
        
        let mut front = VL53L0x::new(front_i2c).unwrap();
        front.set_address(0x20);
        let mut center = VL53L0x::new(center_i2c).unwrap();
        center.set_address(0x21);
    
        front.set_measurement_timing_budget(0).unwrap(); // Set timing budget
        center.set_measurement_timing_budget(0).unwrap(); // Set timing budget
        front.start_continuous(0).unwrap(); // Start continuous mode with 0ms delay
        center.start_continuous(0).unwrap(); // Start continuous mode with 0ms delay
    
        Yaw { front_tof: front, center_tof: center }
    }    
    
    /// Read current distance in continuous mode
    pub fn read_yaw(&mut self) -> Result<Distance, String> {
        print!("Reading the distance & yaw");
        let front_distance = self.front_tof.read_range_continuous_millimeters_blocking().unwrap();
        let center_distance = self.center_tof.read_range_continuous_millimeters_blocking().unwrap();
        let yaw: f64 = (((center_distance as f64) - (front_distance as f64)) / 20.0).atan();

        Ok (Distance {yaw})
        
    }      
}