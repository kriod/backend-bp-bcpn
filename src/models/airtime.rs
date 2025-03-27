use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AirtimeRequestWithPin {
    pub clientTransactionReference: String,
    pub accountNumber: String,
    pub cif: String,
    pub network: String,
    pub phoneNumber: String,
    pub amount: f64,
    pub pin: String,
    pub channelId: String,
    pub securityInfo: String,
    pub isForPoint: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirtimePurchaseResult {
    pub status: Option<String>,
    pub message: Option<String>,
    pub narration: Option<String>,
    pub transactionReference: Option<String>,
    pub platformTransactionReference: Option<String>,
    pub transactionStan: Option<String>,
    pub orinalTxnTransactionDate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirtimePurchaseResponse {
    pub result: Option<AirtimePurchaseResult>,
    pub errorMessage: Option<String>,
    pub errorMessages: Option<Vec<String>>,
    pub hasError: bool,
    pub timeGenerated: Option<String>,
}
