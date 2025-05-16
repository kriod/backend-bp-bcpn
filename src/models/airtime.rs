use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetBillersByCategoryResponse {
    #[serde(rename = "BillerList")]
    pub biller_list: BillerList,
    #[serde(rename = "ResponseCode")]
    pub response_code: String,
    #[serde(rename = "ResponseCodeGrouping")]
    pub response_code_grouping: String,
}

#[derive(Debug, Deserialize)]
pub struct BillerList {
    #[serde(rename = "Count")]
    pub count: u32,
    #[serde(rename = "Category")]
    pub category: Vec<BillerCategory>,
}

#[derive(Debug, Deserialize)]
pub struct BillerCategory {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Billers")]
    pub billers: Vec<Biller>,
}

#[derive(Debug, Deserialize)]
pub struct Biller {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShortName")]
    pub short_name: Option<String>,
    #[serde(rename = "CustomerField1")]
    pub customer_field1: Option<String>,
    #[serde(rename = "LogoUrl")]
    pub logo_url: Option<String>,
    #[serde(rename = "NetworkId")]
    pub network_id: Option<String>,
    #[serde(rename = "ProductCode")]
    pub product_code: Option<String>,
}
