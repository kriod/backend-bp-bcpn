use bills_backend::services::airtime::get_billers_by_category;
use bills_backend::services::billers::get_quickteller_access_token;

#[tokio::test]
async fn test_get_biller_payment_items_success() {
    use bills_backend::services::airtime::get_biller_payment_items;

    dotenvy::dotenv().ok();

    let token_result = get_quickteller_access_token().await;
    assert!(
        token_result.is_ok(),
        "❌ Failed to get access token: {:?}",
        token_result.err()
    );

    let access_token = token_result.unwrap();

    let service_id = 17305; // Airtel

    let result = get_biller_payment_items(&access_token, service_id).await;

    assert!(
        result.is_ok(),
        "❌ Failed to get biller payment items: {:?}",
        result.err()
    );

    let json = result.unwrap();
    println!("✅ Biller payment items: {:#}", json);

    assert!(
        json.get("ResponseCode").is_some(),
        "❌ Missing 'ResponseCode' in response"
    );
}

#[tokio::test]
async fn test_get_billers_by_category_success() {
    dotenvy::dotenv().ok();

    let token_result = get_quickteller_access_token().await;
    assert!(
        token_result.is_ok(),
        "❌ Failed to get access token: {:?}",
        token_result.err()
    );
    let access_token = token_result.unwrap();

    let result = get_billers_by_category(4, &access_token).await;
    assert!(
        result.is_ok(),
        "❌ Failed to get billers for category: {:?}",
        result.err()
    );

    let json = result.unwrap();

    // You can adjust this assertion depending on how the response is structured
    assert!(
        json.get("BillerList").is_some(),
        "❌ Expected 'BillerList' key in response: {:?}",
        json
    );

    println!("✅ Billers by category response: {:?}", json);
}
