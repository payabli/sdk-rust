pub use crate::prelude::*;

/// Response body for queries about chargebacks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryChargebacksResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryChargebacksResponseRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
