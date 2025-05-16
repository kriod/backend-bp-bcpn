use bills_backend::services::billers::get_biller_categories;
use bills_backend::services::billers::get_quickteller_access_token;
use dotenvy;

#[tokio::test]
async fn test_get_biller_categories_success() {
    dotenvy::dotenv().ok();

    let token_result = get_quickteller_access_token().await;
    assert!(
        token_result.is_ok(),
        "❌ Failed to get access token: {:?}",
        token_result.err()
    );

    let token = token_result.unwrap();
    let result = get_biller_categories(&token).await;

    assert!(
        result.is_ok(),
        "❌ Failed to get biller categories: {:?}",
        result.err()
    );

    let parsed = result.unwrap();

    assert_eq!(
        parsed.response_code.as_deref(),
        Some("90000"),
        "❌ Unexpected response code: {:?}",
        parsed.response_code
    );

    assert!(
        !parsed.biller_categories.is_empty(),
        "❌ No biller categories found"
    );

    println!(
        "✅ Retrieved {} biller categories:",
        parsed.biller_categories.len()
    );
    for category in parsed.biller_categories {
        println!(
            "• {} (ID: {}) — {}",
            category.name, category.id, category.description
        );
    }
}

#[tokio::test]
async fn test_get_quickteller_access_token() {
    dotenvy::dotenv().ok();

    let result = get_quickteller_access_token().await;

    assert!(
        result.is_ok(),
        "Expected success, but got error: {:?}",
        result.err()
    );

    let token = result.unwrap();
    assert!(token.len() > 100, "Access token is too short: {}", token);
    assert!(
        token.contains('.'),
        "Access token doesn't look like a JWT: {}",
        token
    );

    println!("✅ Token: {}", token);
}
