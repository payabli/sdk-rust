pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationLogSearchRequest {
    /// The start date for the search.
    #[serde(rename = "startDate")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_date: DateTime<FixedOffset>,
    /// The end date for the search.
    #[serde(rename = "endDate")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_date: DateTime<FixedOffset>,
    /// The type of notification event to filter by.
    #[serde(rename = "notificationEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_event: Option<String>,
    /// Indicates whether the notification was successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<bool>,
    /// The ID of the organization to filter by.
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    /// The ID of the paypoint to filter by.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
}
