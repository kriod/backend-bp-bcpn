use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: i32,
    pub merchant_reference: String,
    pub customer_id: String,
    pub basket_id: String,
    pub amount: i64,
    pub qr_status: String,
    pub confirm_status: String,
    pub timestamp: i64,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub merchant_reference: String,
    pub customer_id: String,
    pub basket_id: String,
    pub amount: i64,
    pub qr_status: String,
    pub confirm_status: String,
    pub timestamp: i64,
    pub user_id: Option<String>,
}
