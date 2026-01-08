pub use crate::prelude::*;

/// Response for line item queries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponseItems {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryResponseItemsRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
