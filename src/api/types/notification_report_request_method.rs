pub use crate::prelude::*;

/// Automated reporting lets you gather critical reports without manually filtering and exporting the data. Get automated daily, weekly, and monthly report for daily sales, ACH returns, settlements, and more. You can send these reports via email or via webhook. See [Automated Reports](/developers/developer-guides/notifications-and-webhooks-overview#automated-reports) for more.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationReportRequestMethod {
    #[serde(rename = "report-email")]
    ReportEmail,
    #[serde(rename = "report-web")]
    ReportWeb,
}
impl fmt::Display for NotificationReportRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReportEmail => "report-email",
            Self::ReportWeb => "report-web",
        };
        write!(f, "{}", s)
    }
}
