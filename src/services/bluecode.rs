use crate::models::bluecode::{BluecodeRegisterRequest, BluecodeRegisterResponse, BluecodeRegisterResponseWrapper};
use crate::models::bluecode::{BluecodeStatusRequest, BluecodeStatusResponseWrapper};
use crate::utils::error::ApiError;
use reqwest::Client;
use std::env;

pub async fn initiate_qr_payment(
    req: BluecodeRegisterRequest,
) -> Result<BluecodeRegisterResponse, ApiError> {
    let client = Client::new();
    let base_url = env::var("BLUECODE_API_BASE_URL")
        .unwrap_or_else(|_| "https://merchant-api.acq.int.bluecode.ng".to_string());

    let full_url = format!("{}/v4/register", base_url);

    let username = env::var("BLUECODE_MERCHANT_ACCESS")
        .map_err(|_| ApiError::EnvVarMissing("BLUECODE_MERCHANT_ACCESS".to_string()))?;
    let password = env::var("BLUECODE_MERCHANT_SECRET")
        .map_err(|_| ApiError::EnvVarMissing("BLUECODE_MERCHANT_SECRET".to_string()))?;

    // 💬 Log what we're sending
    println!("📤 Sending to Bluecode:");
    println!("URL: {}", full_url);
    println!("Payload: {:#?}", req);

    let response = client
        .post(&full_url)
        .basic_auth(username, Some(password))
        .json(&req)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            println!("📥 Status: {}", status);
            println!("📥 Response body: {}", body);

            if status.is_success() {
                let parsed = serde_json::from_str::<BluecodeRegisterResponseWrapper>(&body)
                    .map_err(|_| ApiError::InternalServerError)?;

                Ok(parsed.payment)
            } else {
                Err(ApiError::InternalServerError)
            }
        }
        Err(err) => {
            println!("❌ HTTP error: {}", err);
            Err(ApiError::RequestError(err))
        }
    }
}


pub async fn requery_transaction(merchant_tx_id: String) -> Result<BluecodeStatusResponseWrapper, ApiError> {
    let client = Client::new();
    let base_url = env::var("BLUECODE_API_BASE_URL")
        .unwrap_or_else(|_| "https://merchant-api.acq.int.bluecode.ng".to_string());

    let full_url = format!("{}/v4/status", base_url);

    let username = env::var("BLUECODE_MERCHANT_ACCESS")
        .map_err(|_| ApiError::EnvVarMissing("BLUECODE_MERCHANT_ACCESS".to_string()))?;
    let password = env::var("BLUECODE_MERCHANT_SECRET")
        .map_err(|_| ApiError::EnvVarMissing("BLUECODE_MERCHANT_SECRET".to_string()))?;

    let request_body = BluecodeStatusRequest { merchant_tx_id };

    let response = client
        .post(&full_url)
        .basic_auth(username, Some(password))
        .json(&request_body)
        .send()
        .await?
        .json::<BluecodeStatusResponseWrapper>()
        .await?;

    Ok(response)
}