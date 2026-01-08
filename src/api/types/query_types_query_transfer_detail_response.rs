pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTransferDetailResponse {
    /// List of transfer detail records
    #[serde(rename = "Records")]
    pub records: Vec<TransferDetailRecord>,
    /// Summary of the transfer details query
    #[serde(rename = "Summary")]
    pub summary: QueryTransferSummary,
}
