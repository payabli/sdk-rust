pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDetailResponseSummary {
    #[serde(rename = "serviceFees")]
    pub service_fees: f64,
    #[serde(rename = "transferAmount")]
    pub transfer_amount: f64,
    pub refunds: f64,
    #[serde(rename = "heldAmount")]
    pub held_amount: f64,
    #[serde(rename = "totalRecords")]
    pub total_records: Totalrecords,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
    #[serde(rename = "totalNetAmount")]
    pub total_net_amount: f64,
    #[serde(rename = "totalPages")]
    pub total_pages: Totalpages,
    #[serde(rename = "pageSize")]
    pub page_size: Pagesize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
}
