pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SplitFundingContent {
    /// The accountId for the account the split should be sent to.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Amount from the transaction to sent to this recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// A description for the split.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The entrypoint the split should be sent to.
    #[serde(rename = "recipientEntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_entry_point: Option<String>,
}
