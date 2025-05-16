use crate::utils::error::ApiError;
use reqwest::Client;
use std::env;

pub async fn get_biller_payment_items(
    access_token: &str,
    service_id: u32,
) -> Result<serde_json::Value, ApiError> {
    let base_url = env::var("QUICKTELLER_BASE_URL")
        .map_err(|_| ApiError::EnvVarMissing("QUICKTELLER_BASE_URL".into()))?;
    let terminal_id = env::var("INTERSWITCH_TERMINAL_ID")
        .map_err(|_| ApiError::EnvVarMissing("INTERSWITCH_TERMINAL_ID".into()))?;

    let url = format!("{}/quicktellerservice/api/v5/services/options", base_url);

    let client = Client::new();
    let res = client
        .get(&url)
        .query(&[("serviceid", service_id.to_string())])
        .header("Authorization", format!("Bearer {}", access_token))
        .header("terminalId", terminal_id)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(ApiError::RequestError)?;

    let status = res.status();
    let body = res.text().await.unwrap_or_default();

    if !status.is_success() {
        tracing::error!(
            "❌ Failed to fetch payment items. HTTP {}: {}",
            status,
            body
        );
        return Err(ApiError::HttpError(format!("Status {}: {}", status, body)));
    }

    tracing::info!("✅ Biller payment items retrieved successfully.");
    serde_json::from_str(&body).map_err(|e| {
        tracing::error!("❌ JSON parse error: {}", e);
        ApiError::ParseError(format!("Parse error: {}", e))
    })
}

pub async fn get_billers_by_category(
    category_id: u32,
    access_token: &str,
) -> Result<serde_json::Value, ApiError> {
    let base_url = env::var("QUICKTELLER_BASE_URL")
        .map_err(|_| ApiError::EnvVarMissing("QUICKTELLER_BASE_URL".into()))?;
    let terminal_id = env::var("INTERSWITCH_TERMINAL_ID")
        .map_err(|_| ApiError::EnvVarMissing("INTERSWITCH_TERMINAL_ID".into()))?;

    let url = format!(
        "{}/quicktellerservice/api/v5/services?categoryId={}",
        base_url, category_id
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("terminalId", terminal_id)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(ApiError::RequestError)?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        tracing::error!("❌ Biller fetch failed. HTTP {}: {}", status, body);
        return Err(ApiError::HttpError(format!("Status {}: {}", status, body)));
    }

    tracing::info!(
        "✅ Biller list retrieved for category {}: {}",
        category_id,
        body
    );
    serde_json::from_str(&body).map_err(|e| {
        tracing::error!("❌ JSON parse error: {}", e);
        ApiError::ParseError(format!("Parse error: {}", e))
    })
}
