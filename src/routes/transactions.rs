use crate::models::transactions::{NewTransaction, Transaction};
use axum::{extract::State, routing::get, routing::post, Json, Router};
use sqlx::PgPool;
use tracing::info;
use wiremock::matchers::path;

pub fn transaction_routes(pool: PgPool) -> Router<PgPool> {
    Router::new()
        .route("/", post(store_transaction))
        .route("/", get(get_transactions))
}

async fn get_transactions(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Transaction>>, (axum::http::StatusCode, String)> {
    let rows = sqlx::query_as!(
        Transaction,
        r#"
        SELECT id, merchant_reference, amount, customer_id, basket_id, status, timestamp
        FROM transactions
        ORDER BY timestamp DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rows))
}

async fn store_transaction(
    State(pool): State<PgPool>,
    Json(payload): Json<NewTransaction>,
) -> Result<Json<Transaction>, (axum::http::StatusCode, String)> {
    info!("📥 Saving transaction: {:?}", payload);
    let result = sqlx::query_as!(
        Transaction,
        r#"
        INSERT INTO transactions (merchant_reference, amount, customer_id, basket_id, status, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, merchant_reference, amount, customer_id, basket_id, status, timestamp
        "#,
        payload.merchant_reference,
        payload.amount,
        payload.customer_id,
        payload.basket_id,
        payload.status,
        payload.timestamp
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(result))
}
