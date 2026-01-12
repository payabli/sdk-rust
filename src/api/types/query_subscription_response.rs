pub use crate::prelude::*;

/// Subscription query response body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuerySubscriptionResponse {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<SubscriptionQueryRecords>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
