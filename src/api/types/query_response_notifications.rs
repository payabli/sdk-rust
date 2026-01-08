pub use crate::prelude::*;

/// Response payload for queries related to notifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponseNotifications {
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<QueryResponseNotificationsRecordsItem>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<QuerySummary>,
}
