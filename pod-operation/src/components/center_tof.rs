use {
    vl53l0x::VL53L0x,
    rppal::i2c::I2c,
};

pub struct Center_Tof {
    center_tof: VL53L0x<I2c>,
}

pub struct Distance {
    pub distance: u16,
}

impl Center_Tof {
    pub fn new() -> Self {
        let i2c: I2c = I2c::new().unwrap();

        let mut center = VL53L0x::new(i2c).unwrap();
        center.set_address(0x21);

        center.set_measurement_timing_budget(0).unwrap();
        center.start_continuous(0).unwrap();

        Center_Tof { center }
    }

    pub fn read_distance(&mut self) -> Result<Distance, String> {
        print!("Reading distance...");
        match self.center_tof.read_range_continuous_millimeters_blocking() {
            Ok(distance) => return Ok(Distance { distance }),
            Err(e) => {
                eprintln!("Attempt failed: {:?}", e);
            }
        }
        Err("Failed to read distance".to_string())
    } 
    
}