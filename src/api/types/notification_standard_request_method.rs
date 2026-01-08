pub use crate::prelude::*;

/// Get near-instant notifications via email, SMS, or webhooks for important events like new payment disputes, merchant activations, fraud alerts, approved transactions, settlement history, vendor payouts, and more. Use webhooks with notifications to get real-time updates and automate operations based on key those key events. See [Notifications](/developers/developer-guides/notifications-and-webhooks-overview#notifications) for more.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationStandardRequestMethod {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "web")]
    Web,
}
impl fmt::Display for NotificationStandardRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Email => "email",
            Self::Sms => "sms",
            Self::Web => "web",
        };
        write!(f, "{}", s)
    }
}
