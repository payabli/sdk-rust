pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VCardSummary {
    #[serde(rename = "totalPages")]
    pub total_pages: Totalpages,
    #[serde(rename = "totalRecords")]
    pub total_records: Totalrecords,
    /// Total amount for the records.
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
    /// Total number of active vCards.
    pub totalactive: i64,
    /// Total amount of active vCards.
    pub totalamounteactive: f64,
    /// Total balance of active vCards.
    pub totalbalanceactive: f64,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
}
