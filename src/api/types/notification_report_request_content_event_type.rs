pub use crate::prelude::*;

/// The notification's event name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationReportRequestContentEventType {
    Report,
}
impl fmt::Display for NotificationReportRequestContentEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Report => "Report",
        };
        write!(f, "{}", s)
    }
}
