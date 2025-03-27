use axum::{routing::post, Router};

pub fn payments_routes() -> Router {
    Router::new().route("/", post(process_payment))
}

async fn process_payment() -> &'static str {
    "Payment processed"
}
