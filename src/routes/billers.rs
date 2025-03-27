use axum::{response::Json, routing::get, Router};

use crate::models::billers::Biller;
use crate::services::billers::fetch_billers;

pub fn billers_routes() -> Router {
    Router::new().route("/billers", get(get_billers))
}

async fn get_billers() -> Json<Vec<Biller>> {
    match fetch_billers().await {
        Ok(billers) => Json(billers),
        Err(_) => Json(vec![]),
    }
}
