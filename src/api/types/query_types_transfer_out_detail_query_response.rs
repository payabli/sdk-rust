pub use crate::prelude::*;

/// Response body for queries about outbound transfer details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOutDetailQueryResponse {
    /// Summary information about the transfer details.
    #[serde(rename = "Summary")]
    pub summary: QueryTransferSummary,
    /// List of outbound transfer detail records.
    #[serde(rename = "Records")]
    pub records: Vec<TransferOutDetailRecord>,
}
