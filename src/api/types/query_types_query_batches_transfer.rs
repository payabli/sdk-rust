pub use crate::prelude::*;

/// Transfer details within a batch response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryBatchesTransfer {
    /// The transfer ID.
    #[serde(rename = "TransferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The transfer date.
    #[serde(rename = "TransferDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub transfer_date: Option<DateTime<FixedOffset>>,
    /// The processor used for the transfer.
    #[serde(rename = "Processor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<String>,
    /// The transfer status.
    #[serde(rename = "TransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<i64>,
    /// The gross amount of the transfer.
    #[serde(rename = "GrossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f64>,
    /// The chargeback amount.
    #[serde(rename = "ChargeBackAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_back_amount: Option<f64>,
    /// The returned amount.
    #[serde(rename = "ReturnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_amount: Option<f64>,
    /// The refund amount.
    #[serde(rename = "RefundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<f64>,
    /// The amount being held.
    #[serde(rename = "HoldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_amount: Option<f64>,
    /// The amount that has been released.
    #[serde(rename = "ReleasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_amount: Option<f64>,
    /// The billing fees amount.
    #[serde(rename = "BillingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_amount: Option<f64>,
    /// The third party paid amount.
    #[serde(rename = "ThirdPartyPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_paid_amount: Option<f64>,
    /// The adjustments amount.
    #[serde(rename = "AdjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments_amount: Option<f64>,
    /// The net funded amount.
    #[serde(rename = "NetFundedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_funded_amount: Option<f64>,
}
