use crate::models::airtime::{AirtimePurchaseResponse, AirtimeRequestWithPin};
use crate::services::airtime::purchase_airtime_with_pin;
use crate::utils::error::ApiError;
use crate::utils::logger::log_error;
use axum::response::IntoResponse;
use axum::{routing::post, Json, Router};

pub fn airtime_routes() -> Router {
    Router::new().route("/purchase", post(purchase_airtime)) // Ensure this route is registered
}

async fn purchase_airtime(Json(payload): Json<AirtimeRequestWithPin>) -> impl IntoResponse {
    match purchase_airtime_with_pin(payload).await {
        Ok(response) => Json(response).into_response(),
        Err(ApiError::EnvVarMissing(var_name)) => {
            log_error(&format!("Missing environment variable: {}", var_name));
            Json(format!("Environment variable missing: {}", var_name)).into_response()
        }
        Err(ApiError::RequestError(err)) => {
            log_error(&format!("Request error: {}", err));
            Json(format!("Request error: {}", err)).into_response()
        }
        Err(_) => {
            log_error("An unknown error occurred");
            Json("An unknown error occurred".to_string()).into_response()
        }
    }
}
