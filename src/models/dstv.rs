use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DstvConfirmPaymentRequest {
    pub customer_id: String,
    pub basket_id: String,
    pub amount: u32,
    pub merchant_reference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DstvLookupRequest {
    pub customer_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DstvLookupResponse {
    pub account_name: Option<String>,
    pub customer_id: Option<String>,
    pub message: String,
    pub success: bool,
    pub custom_fields: Option<HashMap<String, String>>,
}
