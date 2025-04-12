use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub merchant_reference: String,
    pub amount: i64,
    pub customer_id: String,
    pub basket_id: String,
    pub status: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTransaction {
    pub merchant_reference: String,
    pub amount: i64,
    pub customer_id: String,
    pub basket_id: String,
    pub status: String,
    pub timestamp: i64,
}
