use {
    vl53l0x::VL53L0x,
    rppal::{i2c::I2c},
};


pub struct Yaw {
    yaw: VL53L0x<I2c>,
}

pub struct Distance {
    pub distance: u16,
}

impl Yaw {
    /// Initialize the sensor for continuous reading
    pub fn new() -> Self {
        let i2c = I2c::new().unwrap();
        let mut yaw = VL53L0x::new(i2c).unwrap();
    
        yaw.set_measurement_timing_budget(0).unwrap(); // Set timing budget
        yaw.start_continuous(0).unwrap(); // Start continuous mode with 0ms delay
    
        Yaw { yaw }
    }    
    
    /// Read current distance in continuous mode
    pub fn read_distance(&mut self) -> Result<Distance, String> {
        print!("Reading distance...");
        match self.yaw.read_range_continuous_millimeters_blocking() {
            Ok(distance) => return Ok(Distance { distance }),
            Err(e) => {
                eprintln!("Attempt failed: {:?}", e);
            }
        }
        Err("Failed to read distance after 3 attempts".to_string())
    }      

    // pub fn read_distance(&mut self) -> Result<Distance, String> {
    //     for attempt in 1..=3 {
    //         println!("Attempt {}: Reading distance...", attempt);
    //         match self.yaw.read_range_continuous_millimeters_blocking() {
    //             Ok(distance) => return Ok(Distance { distance }),
    //             Err(e) => {
    //                 eprintln!("Attempt {} failed: {:?}", attempt, e);
    //                 std::thread::sleep(std::time::Duration::from_millis(300)); // Increase delay to 300ms
    //             }
    //         }
    //     }
    //     Err("Failed to read distance after 3 attempts".to_string())
    // }        
}