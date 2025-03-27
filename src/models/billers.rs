use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Biller {
    pub categoryid: String,
    pub categoryname: String,
    pub categorydescription: String,
    pub billerid: String,
    pub billername: String,
    pub customerfield1: String,
    pub customerfield2: Option<String>,
    pub currencySymbol: String,
    pub logoUrl: Option<String>,
}
