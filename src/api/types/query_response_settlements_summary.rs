pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponseSettlementsSummary {
    /// Funds being held for fraud or risk concerns.
    #[serde(rename = "heldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// Number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Total refunds deducted from the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<f64>,
    /// Service fees are any pass-through fees charged to the customer at the time of payment. These aren't transferred to the merchant when the batch is transferred and funded.
    #[serde(rename = "serviceFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fees: Option<f64>,
    /// The total sum of the settlements in the response.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// The total sum of the settlements in the response.
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    /// Number of pages in the response.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Number of records in the response.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
    /// The transfer amount is the net batch amount plus or minus any returns, refunds, billing and fees items, chargebacks, adjustments, and third party payments. This is the amount from the batch that's transferred to the merchant bank account.
    #[serde(rename = "transferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<f64>,
}
