pub use crate::prelude::*;

/// The kind report to generate. For [automated reports](/developers/developer-guides/notifications-and-webhooks-overview#automated-reports) only.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationContentReportName {
    Transaction,
    Settlement,
    Boarding,
    Returned,
}
impl fmt::Display for NotificationContentReportName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Transaction => "Transaction",
            Self::Settlement => "Settlement",
            Self::Boarding => "Boarding",
            Self::Returned => "Returned",
        };
        write!(f, "{}", s)
    }
}
