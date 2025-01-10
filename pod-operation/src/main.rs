use axum::{routing::get, Router};
use socketioxide::SocketIo;

use crate::components::yaw::Yaw;

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

    let yaw = Yaw::new();
    let sensor_task = tokio::spawn(test::read_distance(yaw));

    tokio::select! {
        _ = server_task => {},
        _ = sensor_task => {},
    }
}