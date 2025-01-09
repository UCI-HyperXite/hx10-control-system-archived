use axum::{routing::get, Router};
use socketioxide::SocketIo;
use tracing_subscriber; 

// mod components;

use crate::components::yaw::Yaw;

mod components;
mod test;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init(); // initialize subscriber for logging

    // Make a layer for Socket connection
    let (layer, _io) = SocketIo::new_layer();

    // Initialize the axum router 
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    // SPAWN SERVER IN SEPARATE TASK 
    let server_task = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("Server error: {}", e);
        }
    });

    let yaw = Yaw::new();

    // spawn sensor reading loop as its own task 
    let sensor_task = tokio::spawn(async move {
        test::read_distance(yaw).await;
    });

    // waits until either server OR sensor task is done
    tokio::select! {
        _ = server_task => {},
        _ = sensor_task => {},
    }
}