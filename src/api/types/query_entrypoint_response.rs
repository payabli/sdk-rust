pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryEntrypointResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryEntrypointResponseRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
