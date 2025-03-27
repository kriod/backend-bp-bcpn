use crate::models::payments::PaymentRequest;
use reqwest::Client;

pub async fn process_payment(payment: PaymentRequest) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = "https://api.example.com/pay";
    let response = client.post(url).json(&payment).send().await?;
    Ok(response.text().await?)
}
