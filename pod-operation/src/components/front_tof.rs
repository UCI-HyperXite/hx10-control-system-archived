use {
    vl53l0x::VL53L0x,
    rppal::i2c::I2c,
};

pub struct Front_Tof {
    front_tof: VL53L0x<I2c>,
}

pub struct Distance {
    pub distance: u16,
}

impl Front_Tof {
    pub fn new() -> Self {
        let i2c: I2c = I2c::new().unwrap();

        let mut front = VL53L0x::new(i2c).unwrap();
        front.set_address(0x20);

        front.set_measurement_timing_budget(0).unwrap();
        front.start_continuous(0).unwrap();

        Front_Tof { front_tof: front }
    }

    pub fn read_distance(&mut self) -> Result<Distance, String> {
        print!("Reading distance...");
        match self.front_tof.read_range_continuous_millimeters_blocking() {
            Ok(distance) => return Ok(Distance { distance }),
            Err(e) => {
                eprintln!("Attempt failed: {:?}", e);
            }
        }
        Err("Failed to read distance after 3 attempts".to_string())
    } 
    
}