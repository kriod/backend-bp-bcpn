use crate::models::dstv::{DstvLookupRequest, DstvLookupResponse};
use crate::utils::error::ApiError;
use base64::{engine::general_purpose, Engine as _};
use quick_xml::de::from_str;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Context, Result};
use quick_xml::se::to_string;

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

pub async fn confirm_dstv_payment(
    merchant_reference: String,
    customer_id: String,
    basket_id: String,
    amount: u32,
) -> Result<String> {
    let xml_payload = PayUVasRequest {
        version: "1.0",
        MerchantId: "test",
        MerchantReference: &merchant_reference,
        TransactionType: "SINGLE",
        VasId: "MCA_ACCOUNT_SQ_NG",
        CountryCode: "NG",
        AmountInCents: amount,
        CustomerId: &customer_id,
        CustomFields: CustomFields {
            field: vec![CustomField {
                key: "BasketId",
                value: &basket_id,
            }],
        },
    };

    let xml = to_string(&xml_payload).context("Failed to serialize XML")?;
    let client = Client::new();

    let response_result = client
        .post("https://mcapi-server.herokuapp.com/Vendor/SinglePayment")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(xml)
        .send()
        .await;

    match response_result {
        Ok(res) => {
            let text = res.text().await.unwrap_or_default();
            Ok(text)
        }
        Err(_) => {
            tracing::warn!("🛑 Initial confirmation failed, falling back to requery");
            requery_dstv_confirmation(&merchant_reference).await
        }
    }
}

async fn requery_dstv_confirmation(reference: &str) -> Result<String> {
    let client = Client::new();
    let url = format!(
        "https://mcapi-server.herokuapp.com/Transactions/Single/{}",
        reference
    );

    let auth = format!(
        "Basic {}",
        general_purpose::STANDARD.encode("test:NeRWNtWQMS")
    );

    let response = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await
        .context("Failed to send requery request")?
        .text()
        .await?;

    #[derive(Debug, Deserialize)]
    struct PayUVasResponse {
        #[serde(rename = "ResultCode")]
        result_code: String,
        #[serde(rename = "ResultMessage")]
        result_message: String,
    }

    let parsed: PayUVasResponse = from_str(&response).context("Failed to parse requery XML")?;

    tracing::info!(?parsed, "📥 Requery DSTV result");

    if parsed.result_code == "00" {
        Ok(response) // success ✅
    } else {
        Err(ApiError::InternalServerError.into())
    }
}


pub async fn lookup_dstv_account(req: DstvLookupRequest) -> Result<DstvLookupResponse, ApiError> {
    let client = Client::new();
    let url = std::env::var("DSTV_LOOKUP_URL")
        .unwrap_or_else(|_| "https://mcapi-demo.herokuapp.com/vendor/lookup".to_string());

    let xml = format!(
        r#"<PayUVasRequest>
            <MerchantId>test</MerchantId>
            <MerchantReference>ref-123</MerchantReference>
            <TransactionType>ACCOUNT_LOOKUP</TransactionType>
            <VasId>MCA_ACCOUNT_SQ_NG</VasId>
            <CountryCode>NG</CountryCode>
            <CustomerId>{}</CustomerId>
        </PayUVasRequest>"#,
        req.customer_id
    );

    let encoded = [("xml", xml)];

    let auth = format!(
        "Basic {}",
        general_purpose::STANDARD.encode("test:NeRWNtWQMS")
    );

    let response = client
        .post(url)
        .header("Authorization", auth)
        .form(&encoded)
        .send()
        .await?
        .text()
        .await?;

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
    let url = std::env::var("DSTV_PAYMENT_URL")
        .unwrap_or_else(|_| "https://mcapi-demo.herokuapp.com/vendor/singlepayment".to_string());

    let xml = format!(
        r#"<PayUVasRequest>
            <MerchantId>test</MerchantId>
            <MerchantReference>{}</MerchantReference>
            <TransactionType>SINGLE_PAYMENT</TransactionType>
            <VasId>MCA_ACCOUNT_SQ_NG</VasId>
            <CountryCode>NG</CountryCode>
            <CustomerId>{}</CustomerId>
            <Amount>{}</Amount>
            <ProductCode>{}</ProductCode>
        </PayUVasRequest>"#,
        req.merchant_reference, req.customer_id, req.amount, req.product_code
    );

    let encoded = [("xml", xml)];

    let auth = format!(
        "Basic {}",
        general_purpose::STANDARD.encode("test:NeRWNtWQMS")
    );

    let response = client
        .post(url)
        .header("Authorization", auth)
        .form(&encoded)
        .send()
        .await?
        .text()
        .await?;

    tracing::info!("✅ Payment response XML: {}", response);
    Ok(response)
}
