use {
    vl53l0x::VL53L0x, // adafruit sensor name 
    rppal::{hal::Delay, i2c::I2c},
};


pub struct Yaw{
    yaw: VL53L0x<I2c>,
}

pub struct Distance{
    pub distance: i32,
}

pub impl Yaw{
    pub fn new() -> Self{
        let i2c = I2c::new().unwrap();
        let mut yaw = VL53L0x::new(i2c);
        yaw.init(&mut Delay::new()).unwrap();

        Yaw {yaw}
    }

    pub fn read_distance(&mut self) -> Distance {
        Distance{
            distance: self.yaw.read_range_continuous_millimeters_blocking().unwrap(),
        }
    }
}

/* 
pub struct VL53L0X {
    vl53l0x_front: VL53L0x<I2c>,
    vl53l0x_center: VL53L0x<I2c>,
}

pub struct Yaw {
    pub distance_front: f32,
    pub distance_center: f32,
    pub yaw: f32,
}

pub impl VL53L0X {
    pub fn new() -> Self{
        let i2c_front = I2c::new().unwrap();
        let i2c_center = I2c::new().unwrap();

        let mut vl53l0x_front = VL53L0x::new(i2c_front);
        let mut vl53l0x_center = VL53L0x::new(i2c_center);

        vl53l0x_front.init(&mut Delay::new()).unwrap();
        vl53l0x_center.init(&mut Delay::new()).unwrap();

        VL53L0X {vl53l0x_front, vl53l0x_center}
    }
}
*/