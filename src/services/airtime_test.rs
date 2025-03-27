use crate::services::airtime::purchase_airtime_with_pin;
use crate::models::airtime::AirtimeRequestWithPin;
use crate::utils::error::ApiError;
use std::env;

#[tokio::test]
async fn test_missing_environment_variable() {
    // Temporarily unset the AIRTIME_API_BASE_URL to simulate the error
    env::remove_var("AIRTIME_API_BASE_URL");

    let request = AirtimeRequestWithPin {
        clientTransactionReference: "TXN001".to_string(),
        accountNumber: "1234567890".to_string(),
        cif: "CIF123".to_string(),
        network: "MTN".to_string(),
        phoneNumber: "08012345678".to_string(),
        amount: 100.0,
        pin: "1234".to_string(),
        channelId: "WEB".to_string(),
        securityInfo: "secure".to_string(),
        isForPoint: false,
    };

    let result = purchase_airtime_with_pin(request).await;

    assert!(matches!(result, Err(ApiError::EnvVarMissing(_))));
}

#[tokio::test]
async fn test_successful_airtime_purchase() {
    // Simulate successful API response (you may mock the API call here)
    let request = AirtimeRequestWithPin {
        clientTransactionReference: "TXN002".to_string(),
        accountNumber: "9876543210".to_string(),
        cif: "CIF456".to_string(),
        network: "MTN".to_string(),
        phoneNumber: "08098765432".to_string(),
        amount: 200.0,
        pin: "5678".to_string(),
        channelId: "WEB".to_string(),
        securityInfo: "secureinfo".to_string(),
        isForPoint: false,
    };

    // Simulate a successful response from the API (mocking for unit test)
    let response = AirtimePurchaseResponse {
        result: Some(AirtimePurchaseResult {
            status: Some("Success".to_string()),
            message: Some("Airtime purchase successful".to_string()),
            narration: Some("Transaction complete".to_string()),
            transactionReference: Some("TXN12345".to_string()),
            platformTransactionReference: Some("PLATFORM123".to_string()),
            transactionStan: Some("STAN123".to_string()),
            orinalTxnTransactionDate: Some("2023-01-01T00:00:00".to_string()),
        }),
        errorMessage: None,
        errorMessages: None,
        hasError: false,
        timeGenerated: Some("2023-01-01T00:00:00".to_string()),
    };

    // Check that the result matches the expected successful response
    assert_eq!(purchase_airtime_with_pin(request).await.unwrap(), response);
}
