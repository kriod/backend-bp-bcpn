use axum::{Json, Router, routing::post};
use axum::response::IntoResponse;
use crate::models::bluecode::{BluecodeStatusResponseWrapper};
use tracing::info;
use serde::Serialize;

#[derive(Serialize)]
struct AckResponse {
    status: &'static str,
}

pub fn bluecode_routes() -> Router {
    Router::new().route("/callback", post(callback_handler))
}

pub async fn callback_handler(Json(payload): Json<BluecodeStatusResponseWrapper>) -> impl IntoResponse {
    info!("📬 Received Bluecode callback: {:?}", payload);

    // TODO: save status to DB, update transaction, etc.
    Json(AckResponse { status: "received" })

}