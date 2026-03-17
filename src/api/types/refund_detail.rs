pub use crate::prelude::*;

/// Object containing details about the refund, including line items and optional split instructions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundDetail {
    /// Array of payment categories/line items describing the amount to be paid. Note: These categories are for information only and aren't validated against the total amount provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PaymentCategories>>,
    /// Array of objects containing split instructions for the refund.
    #[serde(rename = "splitRefunding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_refunding: Option<Vec<SplitFundingRefundContent>>,
}
