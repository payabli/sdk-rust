pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryPayoutTransactionSummary {
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(rename = "totalAuthorized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_authorized: Option<i64>,
    #[serde(rename = "totalAuthorizedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_authorized_amount: Option<f64>,
    #[serde(rename = "totalCanceled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_canceled: Option<i64>,
    #[serde(rename = "totalCanceledAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_canceled_amount: Option<f64>,
    #[serde(rename = "totalCaptured")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_captured: Option<i64>,
    #[serde(rename = "totalCapturedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_captured_amount: Option<f64>,
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    #[serde(rename = "totalOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_open: Option<i64>,
    #[serde(rename = "totalOpenAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_open_amount: Option<f64>,
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    #[serde(rename = "totalPaid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_paid: Option<i64>,
    #[serde(rename = "totalPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_paid_amount: Option<f64>,
    /// Total number of transactions that are currently on hold.
    #[serde(rename = "totalOnHold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_on_hold: Option<i64>,
    /// Total amount of transactions that are currently on hold.
    #[serde(rename = "totalOnHoldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_on_hold_amount: Option<f64>,
    #[serde(rename = "totalProcessing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processing: Option<i64>,
    #[serde(rename = "totalProcessingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processing_amount: Option<f64>,
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
}
