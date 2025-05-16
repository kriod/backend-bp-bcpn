use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetBillerCategoriesResponse {
    #[serde(rename = "BillerCategories")]
    pub biller_categories: Vec<BillerCategory>,

    #[serde(rename = "ResponseCode")]
    pub response_code: Option<String>,

    #[serde(rename = "ResponseCodeGrouping")]
    pub response_code_grouping: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BillerCategory {
    #[serde(rename = "Id")]
    pub id: u32,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Description")]
    pub description: String,
}
