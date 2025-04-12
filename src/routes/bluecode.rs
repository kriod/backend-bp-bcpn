use crate::models::bluecode::BluecodeStatusResponseWrapper;
use axum::response::IntoResponse;
use axum::{routing::post, Json, Router};
use serde::Serialize;
use sqlx::PgPool;
use tracing::info;

#[derive(Serialize)]
struct AckResponse {
    status: &'static str,
}

pub fn bluecode_routes(pool: PgPool) -> Router<PgPool> {
    Router::new()
        .route("/callback", post(callback_handler))
        .with_state(pool)
}

pub async fn callback_handler(
    Json(payload): Json<BluecodeStatusResponseWrapper>,
) -> impl IntoResponse {
    info!("📬 Received Bluecode callback: {:?}", payload);

    // TODO: save status to DB, update transaction, etc.
    Json(AckResponse { status: "received" })
}
