pub use crate::prelude::*;

/// Response body for queries about batches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryBatchesResponse {
    #[serde(rename = "Records")]
    pub records: Vec<QueryBatchesResponseRecordsItem>,
    #[serde(rename = "Summary")]
    pub summary: BatchSummary,
}
