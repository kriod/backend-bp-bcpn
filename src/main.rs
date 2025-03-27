mod routes; 
mod services;
mod models;
mod utils;

use axum::{Router};
use routes::airtime::airtime_routes; 
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;
use dotenvy::dotenv;
use routes::dstv::dstv_routes;
use routes::bluecode::bluecode_routes;




#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Set up the routes
    let app = Router::new()
        .nest("/airtime", airtime_routes()) // Ensure this is correctly nested
        .nest("/dstv", dstv_routes())
        .nest("/bluecode", bluecode_routes());
        

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("🚀 Server running at http://{}", addr);

    // Bind listener and start the server
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
