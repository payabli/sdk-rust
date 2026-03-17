pub use crate::prelude::*;

/// Response body for queries about money out batches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryBatchesOutResponse {
    #[serde(rename = "Records")]
    pub records: Vec<QueryBatchesOutResponseRecordsItem>,
    #[serde(rename = "Summary")]
    pub summary: BatchSummary,
}
