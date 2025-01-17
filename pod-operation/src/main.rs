use axum::{routing::get, Router};
use socketioxide::SocketIo;

use crate::components::yaw::*;

mod components;
mod test;

#[tokio::main]
async fn main() {
    // Set up the Axum server
    let (layer, _io) = SocketIo::new_layer();
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    // Start the server and distance-reading task concurrently
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let front_tof = Front_Tof::new();
    let sensor_task_front = tokio::spawn(test::read_front_tof(front_tof));

    let center_tof = Center_Tof::new();
    let sensor_task_center = tokio::spawn(test::read_center_tof(center_tof));

    tokio::select! {
        _ = server_task => {},
        _ = sensor_task_front => {},
        _ = sensor_task_center => {},
    }
}