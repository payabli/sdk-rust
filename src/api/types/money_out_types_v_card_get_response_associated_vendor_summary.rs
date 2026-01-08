pub use crate::prelude::*;

/// Summary of vendor's billing and transaction status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VCardGetResponseAssociatedVendorSummary {
    /// Number of active bills.
    #[serde(rename = "ActiveBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bills: Option<i64>,
    /// Number of bills pending approval or payment.
    #[serde(rename = "PendingBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_bills: Option<i64>,
    /// Number of bills in transit.
    #[serde(rename = "InTransitBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_bills: Option<i64>,
    /// Number of bills that have been paid.
    #[serde(rename = "PaidBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_bills: Option<i64>,
    /// Number of bills that are overdue.
    #[serde(rename = "OverdueBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdue_bills: Option<i64>,
    /// Number of bills that have been approved.
    #[serde(rename = "ApprovedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_bills: Option<i64>,
    /// Number of bills that have been disapproved.
    #[serde(rename = "DisapprovedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disapproved_bills: Option<i64>,
    /// Total number of bills.
    #[serde(rename = "TotalBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bills: Option<i64>,
    /// Total amount of active bills.
    #[serde(rename = "ActiveBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bills_amount: Option<f64>,
    /// Total amount of pending bills.
    #[serde(rename = "PendingBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_bills_amount: Option<f64>,
    /// Total amount of bills in transit.
    #[serde(rename = "InTransitBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_bills_amount: Option<f64>,
    /// Total amount of paid bills.
    #[serde(rename = "PaidBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_bills_amount: Option<f64>,
    /// Total amount of overdue bills.
    #[serde(rename = "OverdueBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdue_bills_amount: Option<f64>,
    /// Total amount of approved bills.
    #[serde(rename = "ApprovedBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_bills_amount: Option<f64>,
    /// Total amount of rejected bills.
    #[serde(rename = "DisapprovedBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disapproved_bills_amount: Option<f64>,
    /// Total amount of all bills.
    #[serde(rename = "TotalBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bills_amount: Option<f64>,
}
