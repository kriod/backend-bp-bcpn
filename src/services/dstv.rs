use crate::models::dstv::{DstvLookupRequest, DstvLookupResponse};
use crate::utils::error::ApiError;
use base64::{engine::general_purpose, Engine as _};
use quick_xml::de::from_str;
use reqwest::Client;
use serde::Deserialize;

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
    // 👉 Add this for debugging
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

    if let Some(customs) = parsed.custom_fields {
        for field in customs.fields {
            match field.key.as_str() {
                "SURNAME" => account_name = Some(field.value.clone()),
                "DSTV_CUSTOMER_NUMBER" => customer_id = Some(field.value.clone()),
                _ => {}
            }
        }
    }

    Ok(DstvLookupResponse {
        account_name,
        customer_id,
        message: parsed.result_message,
        success: parsed.result_code == "00",
    })
}
