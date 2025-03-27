use axum::{Router, body::Body, http::{Request, StatusCode}};
use axum::routing::{post, get};
use tower::ServiceExt; // for `oneshot`
use bills_backend::routes::bluecode::callback_handler;
use bills_backend::routes::dstv::requery_handler;
use serde_json::json;
use axum::body::to_bytes;



#[tokio::test]
async fn test_bluecode_callback_handler() {
    let app = Router::new()
        .route("/bluecode/callback", post(callback_handler));

    let payload = json!({
        "result": "OK",
        "payment": {
            "state": "APPROVED",
            "merchant_tx_id": "TXN-12345678"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/bluecode/callback")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}


#[tokio::test]
async fn test_requery_handler_stubbed() {
    let app = Router::new()
        .route("/dstv/requery/{merchant_tx_id}", get(requery_handler));

    let request = Request::builder()
        .method("GET")
        .uri("/dstv/requery/test-tx-id")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    // Log status + body for debug
    let status = response.status();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_str = std::str::from_utf8(&body).unwrap_or("<invalid utf8>");

    println!("📦 Status: {}", status);
    println!("📦 Body: {}", body_str);

    assert_eq!(status, StatusCode::OK);
}
