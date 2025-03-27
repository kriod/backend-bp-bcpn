use crate::models::billers::Biller;
use crate::utils::error::ApiError;
use reqwest::{header, Client};
use std::env;

pub async fn fetch_billers() -> Result<Vec<Biller>, ApiError> {
    let client = Client::new();
    let api_url = format!(
        "{}/quickteller/billers",
        env::var("API_BASE_URL").expect("API_BASE_URL must be set")
    );

    // Set headers for authentication
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        "InterswitchAuth SUifhfjdbxbkfhj132hdfhjshfjhsv"
            .parse()
            .unwrap(),
    );
    headers.insert("Signature", "kuTwggg/3gdgdghs=".parse().unwrap());
    headers.insert("Timestamp", "1434455667788".parse().unwrap());
    headers.insert("Nonce", "7333394444423754333".parse().unwrap());
    headers.insert("SignatureMethod", "SHA1".parse().unwrap());
    headers.insert("TerminalID", "3DMO0001".parse().unwrap());

    let response = client
        .get(&api_url)
        .headers(headers)
        .send()
        .await?
        .json::<Vec<Biller>>()
        .await?;

    Ok(response)
}
