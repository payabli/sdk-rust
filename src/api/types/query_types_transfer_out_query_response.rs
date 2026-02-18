pub use crate::prelude::*;

/// Response body for queries about outbound transfers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOutQueryResponse {
    /// Summary information about the transfers.
    #[serde(rename = "Summary")]
    pub summary: TransferOutSummary,
    /// List of outbound transfer records.
    #[serde(rename = "Records")]
    pub records: Vec<TransferOutRecord>,
}