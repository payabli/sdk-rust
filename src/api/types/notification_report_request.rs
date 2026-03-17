pub use crate::prelude::*;

/// Information about the report notification configuration (report-email, report-web).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationReportRequest {
    pub content: NotificationReportRequestContent,
    pub frequency: NotificationReportRequestFrequency,
    /// Automated reporting lets you gather critical reports without manually filtering and exporting the data. Get automated daily, weekly, and monthly report for daily sales, ACH returns, settlements, and more. You can send these reports via email or via webhook. See [Automated Reports](/developers/developer-guides/notifications-and-webhooks-overview#automated-reports) for more.
    pub method: NotificationReportRequestMethod,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Ownerid>,
    #[serde(rename = "ownerType")]
    pub owner_type: Ownertype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Statusnotification>,
    /// Specify the notification target.
    ///
    /// For method=report-email the expected value is a list of email addresses separated by semicolon.
    ///
    /// For method=report-web the expected value is a valid and complete URL. Webhooks support only standard HTTP ports: 80, 443, 8080, or 4443.
    pub target: String,
}
