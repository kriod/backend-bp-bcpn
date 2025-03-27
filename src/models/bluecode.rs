use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BluecodeRegisterResponseWrapper {
    pub result: String,
    pub payment: BluecodeRegisterResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BluecodeRegisterResponse {
    pub merchant_tx_id: String,
    pub checkin_code: String,
    pub state: String,
}
#[derive(Debug, Deserialize)]
pub struct PaymentInitRequest {
    pub amount: i64, // in kobo
}
#[derive(Debug, Serialize)]
pub struct BluecodeRegisterRequest {
    pub merchant_tx_id: String,
    pub branch_ext_id: String,
    pub scheme: String,
    pub requested_amount: i64,
    pub currency: String,
    pub terminal: String,
    pub source: String,
    pub merchant_callback_url: String,
    pub return_url_failure: String,
    pub return_url_success: String,
    pub return_url_cancel: String,
}

#[derive(Debug, Serialize)]
pub struct BluecodeStatusRequest {
    pub merchant_tx_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BluecodeStatusResponseWrapper {
    pub result: String,
    pub payment: BluecodeStatusResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BluecodeStatusResponse {
    pub state: String,
    pub merchant_tx_id: String,
    // Add other fields as needed
}