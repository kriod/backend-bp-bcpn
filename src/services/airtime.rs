use crate::models::airtime::{AirtimePurchaseResponse, AirtimeRequestWithPin};
use crate::utils::error::ApiError;
use reqwest::{header, Client};
use std::env;

pub async fn purchase_airtime_with_pin(
    request: AirtimeRequestWithPin,
) -> Result<AirtimePurchaseResponse, ApiError> {
    // Get environment variables
    let base_url = env::var("AIRTIME_API_BASE_URL")
        .map_err(|_| ApiError::EnvVarMissing("AIRTIME_API_BASE_URL".to_string()))?;

    let api_key = env::var("AIRTIME_API_KEY")
        .map_err(|_| ApiError::EnvVarMissing("AIRTIME_API_KEY".to_string()))?;

    let access_id =
        env::var("ACCESS_ID").map_err(|_| ApiError::EnvVarMissing("ACCESS_ID".to_string()))?;

    // Prepare the URL and headers
    let url = format!("{}/api/Airtime/PurchaseAirtimeWithPin", base_url);

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Ocp-Apim-Subscription-Key", api_key.parse().unwrap());
    headers.insert("AccessId", access_id.parse().unwrap()); // Add Access ID header

    // Create the client and make the request
    let client = Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await?
        .json::<AirtimePurchaseResponse>()
        .await?;

    Ok(response)
}
