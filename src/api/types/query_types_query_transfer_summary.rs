pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTransferSummary {
    /// ACH returns deducted from the batch.
    #[serde(rename = "achReturns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_returns: Option<f64>,
    /// Corrections applied to Billing & Fees charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<f64>,
    /// Charges applied for transactions and services.
    #[serde(rename = "billingFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees: Option<f64>,
    /// Chargebacks deducted from batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargebacks: Option<f64>,
    /// The gross batch amount before deductions.
    #[serde(rename = "grossTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_transfer_amount: Option<f64>,
    /// Previously held funds that have been released after a risk review.
    #[serde(rename = "releaseAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_amount: Option<f64>,
    /// Payments captured in the batch cycle that are deposited separately. For example,  checks or cash payments recorded in the batch but not deposited via Payabli,  or card brands making a direct transfer in certain situations.
    #[serde(rename = "thirdPartyPaid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_paid: Option<f64>,
    /// The gross batch amount minus service fees.
    #[serde(rename = "totalNetAmountTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount_transfer: Option<f64>,
    /// The sum of each splitFundingAmount of each record in the transfer.
    #[serde(rename = "splitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_amount: Option<f64>,
    /// Service fees are any pass-through fees charged to the customer at the time of payment.  These aren't transferred to the merchant when the batch is transferred and funded.
    #[serde(rename = "serviceFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fees: Option<f64>,
    /// The net batch amount is the gross batch amount minus any returns, refunds,
    /// billing and fees items, chargebacks, adjustments, and third party payments.
    #[serde(rename = "netBatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_batch_amount: Option<f64>,
    /// The transfer amount is the net batch amount plus or minus any returns, refunds,  billing and fees items, chargebacks, adjustments, and third party payments.  This is the amount from the batch that is transferred to the merchant bank account.
    #[serde(rename = "transferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<f64>,
    /// Refunds deducted from batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<f64>,
    /// Funds being held for fraud or risk concerns.
    #[serde(rename = "heldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_amount: Option<f64>,
    /// Number of records in the response.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<Totalrecords>,
    /// The total sum of the transfers in the response.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// The total sum of the transfers in the response.
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    /// Number of pages in the response.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<Totalpages>,
    /// Number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    /// Auxiliary validation used internally by payment pages and components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
}
