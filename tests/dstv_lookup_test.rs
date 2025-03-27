use bills_backend::models::dstv::DstvLookupRequest;
use bills_backend::services::dstv::lookup_dstv_account;
use std::sync::{Arc, Mutex};
use wiremock::matchers::{method, path};
use wiremock::Request as WiremockRequest;
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

#[tokio::test]
async fn test_successful_dstv_lookup() {
    let mock_server = MockServer::start().await;

    let fake_response = r#"
        <PayUVasResponse>
            <ResultCode>00</ResultCode>
            <ResultMessage>Success</ResultMessage>
            <CustomFields>
                <Customfield Key="SURNAME" Value="AKINTAYO"/>
                <Customfield Key="DSTV_CUSTOMER_NUMBER" Value="300115673"/>
            </CustomFields>
        </PayUVasResponse>
    "#;

    // capture the request body for debugging
    let seen_body: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let seen_body_clone = seen_body.clone();

    Mock::given(method("POST"))
        .and(path("/vendor/lookup"))
        .respond_with(move |req: &WiremockRequest| {
            let body = std::str::from_utf8(&req.body)
                .unwrap_or("<invalid utf8>")
                .to_string();
            println!("📨 Mock received request body:\n{}", body); // Debug print

            *seen_body_clone.lock().unwrap() = Some(body);

            ResponseTemplate::new(200).set_body_raw(fake_response, "application/xml")
        })
        .mount(&mock_server)
        .await;

    std::env::set_var(
        "DSTV_LOOKUP_URL",
        format!("{}/vendor/lookup", mock_server.uri()),
    );

    let request = DstvLookupRequest {
        customer_id: "300115673".to_string(),
    };

    let result = lookup_dstv_account(request).await;

    match result {
        Ok(r) => {
            assert_eq!(r.account_name.unwrap(), "AKINTAYO");
            assert_eq!(r.customer_id.unwrap(), "300115673");
            assert!(r.success);
            assert_eq!(r.message, "Success");
        }
        Err(e) => {
            println!("❌ Test failed with error: {:?}", e);
            if let Some(body) = &*seen_body.lock().unwrap() {
                println!("📨 Last seen request body: {}", body);
            } else {
                println!("⚠️ No request hit the mock server.");
            }
            panic!("Test failed: {:?}", e);
        }
    }
}
