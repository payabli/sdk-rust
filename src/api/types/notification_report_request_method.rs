pub use crate::prelude::*;

/// Automated reporting lets you gather critical reports without manually filtering and exporting the data. Get automated daily, weekly, and monthly report for daily sales, ACH returns, settlements, and more. You can send these reports via email or via webhook. See [Automated Reports](/developers/developer-guides/notifications-and-webhooks-overview#automated-reports) for more.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationReportRequestMethod {
    ReportEmail,
    ReportWeb,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationReportRequestMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReportEmail => serializer.serialize_str("report-email"),
            Self::ReportWeb => serializer.serialize_str("report-web"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationReportRequestMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "report-email" => Ok(Self::ReportEmail),
            "report-web" => Ok(Self::ReportWeb),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationReportRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReportEmail => write!(f, "report-email"),
            Self::ReportWeb => write!(f, "report-web"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
