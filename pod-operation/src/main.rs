use axum::{routing::get, Router};
use socketioxide::SocketIo;

// mod components;

use crate::components::yaw::Yaw;

mod components;
mod test;

#[tokio::main]
async fn main() {
    // Make a layer for Socket connection
    let (layer, _io) = SocketIo::new_layer();

    // Initialize the
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    axum::serve(listener, app).await.unwrap();


    let yaw: Yaw = Yaw::new();
    tokio::spawn(test::read_distance(yaw));
}
mod.rs

./target
/target
use {
    vl53l0x::VL53L0x,
    rppal::{i2c::I2c},
    // serde::Serialize,
};


pub struct Yaw{
    yaw: VL53L0x<I2c>,
}

// #[derive(Serialize)]
pub struct Distance{
    pub distance: u16,
}

impl Yaw{
    pub fn new() -> Self{
        let i2c = I2c::new().unwrap();
        // let error = "REASON";
        let mut yaw = VL53L0x::new(i2c).unwrap();

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