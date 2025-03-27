use serde::{Deserialize, Serialize};

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
}
