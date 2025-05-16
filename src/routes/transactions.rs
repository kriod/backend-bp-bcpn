use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use tracing::info;

use crate::models::transactions::{NewTransaction, Transaction};

pub fn transaction_routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(store_transaction))
        .route("/", get(get_transactions))
}

// GET /transactions
async fn get_transactions(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Transaction>>, (axum::http::StatusCode, String)> {
    let rows = sqlx::query_as!(
        Transaction,
        r#"
        SELECT id, merchant_reference, customer_id, basket_id, amount, qr_status, confirm_status, timestamp, user_id
        FROM transactions
        ORDER BY timestamp DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(rows))
}

// POST /transactions
async fn store_transaction(
    State(pool): State<PgPool>,
    Json(new_tx): Json<NewTransaction>,
) -> Result<Json<Transaction>, (axum::http::StatusCode, String)> {
    info!("📥 Saving transaction: {:?}", new_tx);

    let inserted = sqlx::query_as!(
        Transaction,
        r#"
        INSERT INTO transactions (merchant_reference, customer_id, basket_id, amount, qr_status, confirm_status, timestamp, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, merchant_reference, customer_id, basket_id, amount, qr_status, confirm_status, timestamp, user_id
        "#,
        new_tx.merchant_reference,
        new_tx.customer_id,
        new_tx.basket_id,
        new_tx.amount,
        new_tx.qr_status,
        new_tx.confirm_status,
        new_tx.timestamp,
        new_tx.user_id,
    )
    .fetch_one(&pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(inserted))
}

fn internal_error<E>(err: E) -> (axum::http::StatusCode, String)
where
    E: std::fmt::Display,
{
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string(),
    )
}
