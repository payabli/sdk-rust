pub use crate::prelude::*;

/// Vendor bill summary statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendorResponseSummary {
    #[serde(rename = "ActiveBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bills: Option<i64>,
    #[serde(rename = "PendingBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_bills: Option<i64>,
    #[serde(rename = "InTransitBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_bills: Option<i64>,
    #[serde(rename = "PaidBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_bills: Option<i64>,
    #[serde(rename = "OverdueBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdue_bills: Option<i64>,
    #[serde(rename = "ApprovedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_bills: Option<i64>,
    #[serde(rename = "DisapprovedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disapproved_bills: Option<i64>,
    #[serde(rename = "TotalBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bills: Option<i64>,
    #[serde(rename = "ActiveBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bills_amount: Option<f64>,
    #[serde(rename = "PendingBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_bills_amount: Option<f64>,
    #[serde(rename = "InTransitBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_bills_amount: Option<f64>,
    #[serde(rename = "PaidBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_bills_amount: Option<f64>,
    #[serde(rename = "OverdueBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdue_bills_amount: Option<f64>,
    #[serde(rename = "ApprovedBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_bills_amount: Option<f64>,
    #[serde(rename = "DisapprovedBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disapproved_bills_amount: Option<f64>,
    #[serde(rename = "TotalBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bills_amount: Option<f64>,
}
