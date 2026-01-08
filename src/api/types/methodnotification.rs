pub use crate::prelude::*;

/// Method to use to send the notification to the target.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Methodnotification {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "report-email")]
    ReportEmail,
    #[serde(rename = "report-web")]
    ReportWeb,
}
impl fmt::Display for Methodnotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Email => "email",
            Self::Sms => "sms",
            Self::Web => "web",
            Self::ReportEmail => "report-email",
            Self::ReportWeb => "report-web",
        };
        write!(f, "{}", s)
    }
}
