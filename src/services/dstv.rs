use crate::models::dstv::{DstvLookupRequest, DstvLookupResponse};
use crate::utils::error::ApiError;
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize)]
#[serde(rename = "PayUVasRequest")]
struct PayUVasRequest<'a> {
    #[serde(rename = "@Ver")]
    version: &'a str,
    MerchantId: &'a str,
    MerchantReference: &'a str,
    TransactionType: &'a str,
    VasId: &'a str,
    CountryCode: &'a str,
    AmountInCents: u32,
    CustomerId: &'a str,
    CustomFields: CustomFields<'a>,
}

#[derive(Serialize)]
struct CustomFields<'a> {
    #[serde(rename = "Customfield")]
    field: Vec<CustomField<'a>>,
}

#[derive(Serialize)]
struct CustomField<'a> {
    #[serde(rename = "@Key")]
    key: &'a str,
    #[serde(rename = "@Value")]
    value: &'a str,
}

fn get_dstv_auth() -> String {
    std::env::var("DSTV_AUTH").unwrap_or_else(|_| {
        format!(
            "Basic {}",
            general_purpose::STANDARD.encode("test:NeRWNtWQMS")
        )
    })
}

pub async fn confirm_dstv_payment(
    merchant_reference: String,
    customer_id: String,
    basket_id: String,
    amount: u32,
) -> Result<String> {
    let xml_payload = PayUVasRequest {
        version: "1.0",
        MerchantId: "Bluecode",
        MerchantReference: &merchant_reference,
        TransactionType: "SINGLE",
        VasId: "MCA_ACCOUNT_SQ_NG",
        CountryCode: "NG",
        AmountInCents: amount / 100,
        CustomerId: &customer_id,
        CustomFields: CustomFields {
            field: vec![CustomField {
                key: "BasketId",
                value: &basket_id,
            }],
        },
    };

    let xml_string = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>{}"#,
        to_string(&xml_payload).context("Failed to serialize XML")?
    );

    tracing::info!("📤 Final XML body:\n{}", xml_string);

    let client = Client::new();
    let auth = get_dstv_auth();

    let encoded = [("xml", xml_string.clone())];

    let base_url = std::env::var("DSTV_BASE_URL")
        .unwrap_or_else(|_| "https://mcapi-demo.herokuapp.com".to_string());
    let url = format!("{}/vendor/singlepayment", base_url);

    let res = client
        .post(&url) // lowercase like curl
        .header("Content-Type", "application/x-www-form-urlencoded") // explicit content-type
        .header("Authorization", auth)
        .form(&encoded)
        .send()
        .await;

    match res {
        Ok(res) => {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();

            tracing::info!("📡 HTTP Status: {}", status);
            tracing::info!("📨 Raw XML from Multichoice:\n{}", text);

            if status.is_success() && !text.trim().is_empty() {
                Ok(text)
            } else {
                tracing::warn!(
                    "❌ Confirmation failed. Status: {} - Body: {}",
                    status,
                    text
                );
                tracing::warn!("🔁 Falling back to requery...");
                requery_dstv_confirmation(&merchant_reference).await
            }
        }
        Err(err) => {
            tracing::error!("❌ HTTP Request failed: {:?}", err);
            tracing::warn!("🔁 Falling back to requery due to network error...");
            requery_dstv_confirmation(&merchant_reference).await
        }
    }
}

use crate::models::dstv::DstvConfirmPaymentRequest;
use axum::{extract::State, Json};
use sqlx::PgPool;

pub async fn retry_dstv_confirmation(
    State(_pool): State<PgPool>, // You can extend this if DB is needed
    Json(body): Json<DstvConfirmPaymentRequest>,
) -> impl axum::response::IntoResponse {
    tracing::info!("🔁 Manual retry DSTV payment with: {:?}", body);

    match confirm_dstv_payment(
        body.merchant_reference.clone(),
        body.customer_id.clone(),
        body.basket_id.clone(),
        body.amount,
    )
    .await
    {
        Ok(xml_response) => {
            tracing::info!("✅ Manual retry success: {}", xml_response);
            Json({
                serde_json::json!({
                    "success": true,
                    "message": "Manual confirmation sent.",
                    "raw_xml": xml_response
                })
            })
        }
        Err(err) => {
            tracing::error!("❌ Manual retry failed: {:?}", err);
            Json({
                serde_json::json!({
                    "success": false,
                    "message": "Manual retry failed."
                })
            })
        }
    }
}

#[derive(Debug, Deserialize)]
struct RequeryItem {
    merchantreference: String,
    smartcard: String,
    status: i32,
    basketid: String,
}

async fn requery_dstv_confirmation(reference: &str) -> Result<String> {
    let client = Client::new();
    let base_url = std::env::var("DSTV_BASE_URL")
        .unwrap_or_else(|_| "https://mcapi-demo.herokuapp.com".to_string());
    let url = format!("{}/transactions/single/{}", base_url, reference);
    //let url = format!("https://mcapi-demo.herokuapp.com/transactions/single/{}",reference);

    let auth = get_dstv_auth();
    let response = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await
        .context("Failed to send requery request")?
        .text()
        .await?;

    tracing::info!("📥 Raw requery response: {}", response);

    if response.trim().is_empty() {
        tracing::error!("❌ Requery response was empty");
        return Err(anyhow::anyhow!("Empty requery response"));
    }

    let items: Vec<RequeryItem> =
        serde_json::from_str(&response).context("Failed to parse requery JSON")?;

    if let Some(item) = items.first() {
        tracing::info!("✅ Requery result: {:?}", item);
        if item.status == 1 {
            // ✅ Success
            Ok(response)
        } else if item.status == -1 {
            // ⏳ Pending, but not a hard failure
            Err(anyhow::anyhow!("Transaction is still pending"))
        } else {
            // ❌ Failure
            Err(anyhow::anyhow!(
                "Requery returned failed status: {}",
                item.status
            ))
        }
    } else {
        Err(anyhow::anyhow!("Requery returned empty array"))
    }
}

pub async fn lookup_dstv_account(req: DstvLookupRequest) -> Result<DstvLookupResponse, ApiError> {
    let client = Client::new();
    let merchant_id = std::env::var("DSTV_MERCHANT_ID").unwrap_or_else(|_| "test".to_string());
    let base_url = std::env::var("DSTV_BASE_URL")
        .unwrap_or_else(|_| "https://mcapi-demo.herokuapp.com".to_string());
    let url = format!("{}/vendor/lookup", base_url);

    //let url = std::env::var("DSTV_LOOKUP_URL").unwrap_or_else(|_| "https://mcapi-demo.herokuapp.com/vendor/lookup".to_string());

    let xml = format!(
        r#"<PayUVasRequest>
            <MerchantId>{}</MerchantId>
            <MerchantReference>ref-334592934</MerchantReference>
            <TransactionType>ACCOUNT_LOOKUP</TransactionType>
            <VasId>MCA_ACCOUNT_SQ_NG</VasId>
            <CountryCode>NG</CountryCode>
            <CustomerId>{}</CustomerId>
        </PayUVasRequest>"#,
        merchant_id, req.customer_id
    );

    tracing::info!("🧾  URL: {}", url);
    tracing::info!("🧾 Merchant ID: {}", merchant_id);
    tracing::info!("🧾 XML: {}", xml);

    let encoded = [("xml", xml)];

    let auth = get_dstv_auth();

    let response = client
        .post(url)
        .header("Authorization", auth)
        .form(&encoded)
        .send()
        .await?
        .text()
        .await?;

    if response.trim().is_empty() {
        tracing::warn!("Requery response is empty – likely a backend outage or invalid request");
        return Err(ApiError::InternalServerError.into());
    }

    println!("💬 DSTV API Raw XML Response:\n{}", response);

    #[derive(Debug, Deserialize)]
    struct CustomField {
        #[serde(rename = "@Key")]
        key: String,
        #[serde(rename = "@Value")]
        value: String,
    }

    #[derive(Debug, Deserialize)]
    struct PayUVasResponse {
        #[serde(rename = "ResultCode")]
        result_code: String,
        #[serde(rename = "ResultMessage")]
        result_message: String,
        #[serde(rename = "CustomFields")]
        custom_fields: Option<CustomFields>,
    }

    #[derive(Debug, Deserialize)]
    struct CustomFields {
        #[serde(rename = "Customfield")]
        fields: Vec<CustomField>,
    }

    let parsed: PayUVasResponse = from_str(&response).map_err(|e| {
        println!("❌ XML Parse Error: {:?}", e);
        ApiError::InternalServerError
    })?;

    let mut account_name = None;
    let mut customer_id = None;
    let mut extracted_fields = HashMap::new();

    if let Some(customs) = parsed.custom_fields {
        for field in customs.fields {
            extracted_fields.insert(field.key.clone(), field.value.clone());

            match field.key.as_str() {
                "SURNAME" => account_name = Some(field.value.clone()),
                "DSTV_CUSTOMER_NUMBER" => customer_id = Some(field.value.clone()),
                _ => {}
            }
        }
    }
    tracing::info!("✅ Custom Fields: {:?}", extracted_fields);

    Ok(DstvLookupResponse {
        account_name,
        customer_id,
        message: "Success".to_string(),
        success: true,
        custom_fields: Some(extracted_fields),
    })
}

#[derive(Serialize)]
pub struct SinglePaymentRequest {
    pub amount: i64,
    pub customer_id: String,
    pub product_code: String,
    pub merchant_reference: String,
}

pub async fn pay_dstv_bill(req: SinglePaymentRequest) -> Result<String, ApiError> {
    let client = Client::new();
    let merchant_id = std::env::var("DSTV_MERCHANT_ID").unwrap_or_else(|_| "test".to_string());

    let base_url =
        env::var("DSTV_BASE_URL").map_err(|_| ApiError::EnvVarMissing("DSTV_BASE_URL".into()))?;
    let url = format!("{}/vendor/singlepayment", base_url);
    //let url = std::env::var("DSTV_PAYMENT_URL").unwrap_or_else(|_| "https://mcapi-demo.herokuapp.com/vendor/singlepayment".to_string());

    let xml = format!(
        r#"<PayUVasRequest>
            <MerchantId>{}</MerchantId>
            <MerchantReference>{}</MerchantReference>
            <TransactionType>SINGLE_PAYMENT</TransactionType>
            <VasId>MCA_ACCOUNT_SQ_NG</VasId>
            <CountryCode>NG</CountryCode>
            <CustomerId>{}</CustomerId>
            <Amount>{}</Amount>
            <ProductCode>{}</ProductCode>
        </PayUVasRequest>"#,
        merchant_id, req.merchant_reference, req.customer_id, req.amount, req.product_code
    );

    use std::fs;
    fs::write("debug_outgoing.xml", &xml).unwrap();

    let encoded = [("xml", xml)];

    let auth = get_dstv_auth();

    let response = client
        .post(url)
        .header("Authorization", auth)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&encoded)
        .send()
        .await?
        .text()
        .await?;

    tracing::info!("✅ Payment response XML: {}", response);
    Ok(response)
}
