pub use crate::prelude::*;

/// Request for searchNotificationLogs (body + query parameters)
///
/// Request type for the SearchNotificationLogsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SearchNotificationLogsRequest {
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    /// The page number to retrieve. Defaults to 1 if not provided.
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub body: NotificationLogSearchRequest,
}
