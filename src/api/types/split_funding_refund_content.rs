pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SplitFundingRefundContent {
    /// The accountId for the account the transaction was routed to.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The amount to refund to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Refund description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The entrypoint the transaction belongs to.
    #[serde(rename = "originationEntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_entry_point: Option<String>,
}
