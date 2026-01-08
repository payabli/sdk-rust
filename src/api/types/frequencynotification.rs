pub use crate::prelude::*;

/// Frequency for notification.
/// For notifications using the *email* ,*sms*, or *web* `method`, the allowed values are:
/// - `untilcancelled`
/// - `one-time`
///
/// For notifications using the *report-email* or *report-web* `method`, the allowed values are:
/// - `one-time`
/// - `daily`
/// - `weekly`
/// - `biweekly`
/// - `monthly`
/// - `quarterly`
/// - `semiannually`
/// - `annually`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Frequencynotification {
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
    #[serde(rename = "untilcancelled")]
    Untilcancelled,
}
impl fmt::Display for Frequencynotification {
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
            Self::Untilcancelled => "untilcancelled",
        };
        write!(f, "{}", s)
    }
}
