pub use crate::prelude::*;

/// Response body for queries about batch details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryBatchesDetailResponse {
    #[serde(rename = "Records")]
    pub records: Vec<BatchDetailResponseRecord>,
    #[serde(rename = "Summary")]
    pub summary: BatchDetailResponseSummary,
}
