pub use crate::prelude::*;

/// The kind report to generate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationReportRequestContentReportName {
    Transaction,
    Settlement,
    Boarding,
    Returned,
}
impl fmt::Display for NotificationReportRequestContentReportName {
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
