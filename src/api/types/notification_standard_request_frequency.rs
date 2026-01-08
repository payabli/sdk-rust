pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationStandardRequestFrequency {
    #[serde(rename = "one-time")]
    OneTime,
    #[serde(rename = "untilcancelled")]
    Untilcancelled,
}
impl fmt::Display for NotificationStandardRequestFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OneTime => "one-time",
            Self::Untilcancelled => "untilcancelled",
        };
        write!(f, "{}", s)
    }
}
