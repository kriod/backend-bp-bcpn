use crate::models::billers::{BillerCategory, GetBillerCategoriesResponse};
use crate::utils::error::ApiError;
use base64::engine::general_purpose;
use base64::Engine;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn get_quickteller_access_token() -> Result<String, ApiError> {
    println!(
        "CLIENT_ID = {:?}, SECRET_KEY = {:?}",
        std::env::var("QUICKTELLER_CLIENT_ID"),
        std::env::var("QUICKTELLER_SECRET_KEY")
    );

    let client_id = env::var("QUICKTELLER_CLIENT_ID")
        .map_err(|_| ApiError::EnvVarMissing("QUICKTELLER_CLIENT_ID".into()))?;
    let secret_key = env::var("QUICKTELLER_SECRET_KEY")
        .map_err(|_| ApiError::EnvVarMissing("QUICKTELLER_SECRET_KEY".into()))?;

    let credentials = format!("{}:{}", client_id, secret_key);
    let encoded = general_purpose::STANDARD.encode(credentials);
    let auth_header = format!("Basic {}", encoded);

    let client = Client::new();
    let res = client
        .post("https://apps.qa.interswitchng.com/passport/oauth/token")
        .header("Authorization", &auth_header)
        .form(&[("grant_type", "client_credentials"), ("scope", "profile")])
        .send()
        .await
        .map_err(ApiError::RequestError)?;

    // 🔍 Get raw response as text so we can inspect it
    let text = res
        .text()
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    tracing::error!("❌ Full token response: {}", text);

    // Try to extract access_token
    let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
        tracing::error!("❌ Failed to parse token response JSON: {}", e);
        ApiError::InternalServerError
    })?;

    let access_token = json["access_token"].as_str().ok_or_else(|| {
        tracing::error!("❌ Missing `access_token` in JSON response");
        ApiError::InternalServerError
    })?;

    Ok(access_token.to_string())
}

pub async fn get_biller_categories(
    access_token: &str,
) -> Result<GetBillerCategoriesResponse, ApiError> {
    let base_url = env::var("QUICKTELLER_BASE_URL")
        .map_err(|_| ApiError::EnvVarMissing("QUICKTELLER_BASE_URL".into()))?;
    let url = format!("{}/quicktellerservice/api/v5/services/categories", base_url);

    let client = Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(ApiError::RequestError)?;

    let status = res.status();
    let body = res.text().await.unwrap_or_default();

    if !status.is_success() {
        tracing::error!(
            "❌ Failed to fetch biller categories. HTTP {}: {}",
            status,
            body
        );
        return Err(ApiError::HttpError(format!("Status {}: {}", status, body)));
    }

    tracing::info!("✅ Biller categories retrieved.");

    serde_json::from_str(&body).map_err(|e| {
        tracing::error!("❌ JSON parse error: {}", e);
        ApiError::ParseError(format!("Parse error: {}", e))
    })
}
