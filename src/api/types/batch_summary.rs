pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// Number of records on each response page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Total amount for the records.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Total net amount for the records.
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    /// Total number of pages in response.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Total number of records in response.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
}
