// src/routes/dstv.rs
use crate::models::bluecode::{
    BluecodeRegisterRequest, BluecodeRegisterResponse, PaymentInitRequest,
};
use crate::models::bluecode::{BluecodeStatusResponse, BluecodeStatusResponseWrapper};
use crate::models::dstv::{DstvConfirmPaymentRequest, DstvLookupRequest, DstvLookupResponse};
use crate::services::bluecode::{initiate_qr_payment, requery_transaction};
use crate::services::dstv::{confirm_dstv_payment, lookup_dstv_account};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct DstvConfirmPaymentResponse {
    success: bool,
    raw_xml: Option<String>,
    message: Option<String>,
}

pub fn dstv_routes(pool: PgPool) -> Router<PgPool> {
    Router::new()
        .route("/lookup", post(lookup_handler))
        .route("/initiate-payment", post(initiate_payment))
        .route("/requery/{merchant_tx_id}", get(requery_handler))
        .route("/confirm-payment", post(confirm_payment_handler))
        .with_state(pool)
}

pub async fn confirm_payment_handler(
    Json(body): Json<DstvConfirmPaymentRequest>,
) -> impl IntoResponse {
    tracing::info!(?body, "📥 Received DSTV confirm-payment request");
    tracing::info!("🛰 Confirming with: {:?}", body);

    match confirm_dstv_payment(
        body.merchant_reference.clone(),
        body.customer_id.clone(),
        body.basket_id.clone(),
        body.amount,
    )
    .await
    {
        Ok(xml_response) => {
            tracing::info!("✅ DSTV confirmation success");
            axum::Json(DstvConfirmPaymentResponse {
                success: true,
                raw_xml: Some(xml_response),
                message: Some("Payment confirmed successfully".to_string()),
            })
            .into_response()
        }
        Err(err) => {
            tracing::error!(?err, "❌ Failed to confirm DSTV payment");
            axum::Json(DstvConfirmPaymentResponse {
                success: false,
                raw_xml: None,
                message: Some("DSTV payment confirmation failed".to_string()),
            })
            .into_response()
        }
    }
}

async fn initiate_payment(Json(payload): Json<PaymentInitRequest>) -> impl IntoResponse {
    let merchant_tx_id = format!("TXN-{}", Uuid::new_v4());

    let req = BluecodeRegisterRequest {
        merchant_tx_id,
        branch_ext_id: env::var("BLUECODE_BRANCH_EXT_ID").unwrap_or_default(),
        scheme: env::var("BLUECODE_SCHEME").unwrap_or("blue_code".into()),
        requested_amount: payload.amount,
        currency: env::var("BLUECODE_CURRENCY").unwrap_or("NGN".into()),
        terminal: env::var("BLUECODE_TERMINAL").unwrap_or("POS001".into()),
        source: env::var("BLUECODE_SOURCE").unwrap_or("web".into()),
        merchant_callback_url: env::var("BLUECODE_CALLBACK_URL").unwrap_or_default(),
        return_url_failure: env::var("BLUECODE_REDIRECT_URL").unwrap_or_default(),
        return_url_success: env::var("BLUECODE_SUCESS_URL").unwrap_or_default(),
        return_url_cancel: env::var("BLUECODE_CANCEL_URL").unwrap_or_default(),
    };

    match initiate_qr_payment(req).await {
        Ok(response) => Json(response).into_response(),
        Err(_) => Json(BluecodeRegisterResponse {
            merchant_tx_id: "".into(),
            checkin_code: "".into(),
            state: "FAILED".into(),
        })
        .into_response(),
    }
}

async fn lookup_handler(Json(payload): Json<DstvLookupRequest>) -> impl IntoResponse {
    match lookup_dstv_account(payload).await {
        Ok(data) => Json(data).into_response(),
        Err(_) => Json(DstvLookupResponse {
            account_name: None,
            customer_id: None,
            message: "Lookup failed".to_string(),
            success: false,
            custom_fields: None,
        })
        .into_response(),
    }
}

pub async fn requery_handler(
    axum::extract::Path(merchant_tx_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    match requery_transaction(merchant_tx_id).await {
        Ok(response) => Json(response).into_response(),
        Err(_) => Json(BluecodeStatusResponseWrapper {
            result: "ERROR".into(),
            payment: BluecodeStatusResponse {
                state: "UNKNOWN".into(),
                merchant_tx_id: "".into(),
            },
        })
        .into_response(),
    }
}
