pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReissuePayoutResponseData {
    /// The transaction ID of the newly created payout.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// The status of the new transaction.
    pub status: String,
    /// The transaction ID of the original payout that was reissued.
    #[serde(rename = "originalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction_id: Option<String>,
}
