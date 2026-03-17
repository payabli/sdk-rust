pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTransferResponse {
    /// Summary information about the transfers.
    #[serde(rename = "Summary")]
    pub summary: QueryTransferSummary,
    /// List of transfer transaction records.
    #[serde(rename = "Records")]
    pub records: Vec<TransactionQueryRecords>,
}
