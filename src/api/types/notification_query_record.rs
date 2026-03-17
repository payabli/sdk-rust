pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationQueryRecord {
    /// Notification content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<NotificationContent>,
    /// Timestamp of when notification was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequencynotification>,
    /// Timestamp of when notification was last updated.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Methodnotification>,
    #[serde(rename = "notificationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<NotificationId>,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Ownerid>,
    /// Name of entity owner of notification.
    #[serde(rename = "ownerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<Ownertype>,
    /// Custom descriptor of source of notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Statusnotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}
