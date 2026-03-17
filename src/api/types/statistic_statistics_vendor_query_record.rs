pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatisticsVendorQueryRecord {
    /// Statistical grouping identifier
    #[serde(rename = "statX")]
    pub stat_x: String,
    /// Number of active transactions
    pub active: i64,
    /// Volume of active transactions
    #[serde(rename = "activeVolume")]
    pub active_volume: f64,
    /// Number of transactions sent to approval
    #[serde(rename = "sentToApproval")]
    pub sent_to_approval: i64,
    /// Volume of transactions sent to approval
    #[serde(rename = "sentToApprovalVolume")]
    pub sent_to_approval_volume: f64,
    /// Number of transactions to approval
    #[serde(rename = "toApproval")]
    pub to_approval: i64,
    /// Volume of transactions to approval
    #[serde(rename = "toApprovalVolume")]
    pub to_approval_volume: f64,
    /// Number of approved transactions
    pub approved: i64,
    /// Volume of approved transactions
    #[serde(rename = "approvedVolume")]
    pub approved_volume: f64,
    /// Number of disapproved transactions
    pub disapproved: i64,
    /// Volume of disapproved transactions
    #[serde(rename = "disapprovedVolume")]
    pub disapproved_volume: f64,
    /// Number of cancelled transactions
    pub cancelled: i64,
    /// Volume of cancelled transactions
    #[serde(rename = "cancelledVolume")]
    pub cancelled_volume: f64,
    /// Number of transactions in transit
    #[serde(rename = "inTransit")]
    pub in_transit: i64,
    /// Volume of transactions in transit
    #[serde(rename = "inTransitVolume")]
    pub in_transit_volume: f64,
    /// Number of paid transactions
    pub paid: i64,
    /// Volume of paid transactions
    #[serde(rename = "paidVolume")]
    pub paid_volume: f64,
}
