pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationReportRequestFrequency {
    #[serde(rename = "one-time")]
    OneTime,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "biweekly")]
    Biweekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "quarterly")]
    Quarterly,
    #[serde(rename = "semiannually")]
    Semiannually,
    #[serde(rename = "annually")]
    Annually,
}
impl fmt::Display for NotificationReportRequestFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OneTime => "one-time",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Biweekly => "biweekly",
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::Semiannually => "semiannually",
            Self::Annually => "annually",
        };
        write!(f, "{}", s)
    }
}
