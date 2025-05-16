use axum::http::{header, Method};
use axum::Router;

use bills_backend::routes::bluecode::bluecode_routes;
use bills_backend::routes::dstv::dstv_routes;
use bills_backend::routes::transactions::transaction_routes;

use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    // ✅ DB Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Failed to connect to DB");

    // ✅ Global CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    // ✅ Top-level router WITH state: PgPool
    let app = Router::<PgPool>::new()
        .nest("/dstv", dstv_routes(pool.clone()))
        .nest("/bluecode", bluecode_routes(pool.clone()))
        .nest("/transactions", transaction_routes())
        .layer(cors)
        .with_state(pool); // 👈 attaches the PgPool to all routes

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
