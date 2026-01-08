pub use crate::prelude::*;

/// Information about the standard notification configuration (email, sms, web).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationStandardRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<NotificationStandardRequestContent>,
    pub frequency: NotificationStandardRequestFrequency,
    /// Get near-instant notifications via email, SMS, or webhooks for important events like new payment disputes, merchant activations, fraud alerts, approved transactions, settlement history, vendor payouts, and more. Use webhooks with notifications to get real-time updates and automate operations based on key those key events. See [Notifications](/developers/developer-guides/notifications-and-webhooks-overview#notifications) for more.
    pub method: NotificationStandardRequestMethod,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Ownerid>,
    #[serde(rename = "ownerType")]
    pub owner_type: Ownertype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Statusnotification>,
    /// Specify the notification target.
    ///
    /// - For method=email the expected value is a list of email addresses separated by semicolon.
    /// - For method=sms the expected value is a list of phone numbers separated by semicolon.
    /// - For method=web the expected value is a valid and complete URL. Webhooks support only standard HTTP ports: 80, 443, 8080, or 4443.
    pub target: String,
}
