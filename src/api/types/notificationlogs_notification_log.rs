pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationLog {
    /// The unique identifier for the notification.
    pub id: Uuid,
    /// The ID of the organization that the notification belongs to.
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    /// The ID of the paypoint that the notification is related to.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// The event that triggered the notification.
    #[serde(rename = "notificationEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_event: Option<String>,
    /// The target URL for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// The HTTP response status of the notification.
    #[serde(rename = "responseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<String>,
    /// Indicates whether the notification was successful.
    pub success: bool,
    /// Contains the body of the notification.
    #[serde(rename = "jobData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_data: Option<String>,
    /// The date and time when the notification was created.
    #[serde(rename = "createdDate")]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_date: DateTime<Utc>,
    /// The date and time when the notification was successfully delivered.
    #[serde(rename = "successDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub success_date: Option<DateTime<Utc>>,
    /// The date and time when the notification last failed.
    #[serde(rename = "lastFailedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_failed_date: Option<DateTime<Utc>>,
    /// Indicates whether the notification is currently in progress.
    #[serde(rename = "isInProgress")]
    pub is_in_progress: bool,
}
