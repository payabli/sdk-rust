pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationReportRequestFrequency {
    OneTime,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Semiannually,
    Annually,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationReportRequestFrequency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneTime => serializer.serialize_str("one-time"),
            Self::Daily => serializer.serialize_str("daily"),
            Self::Weekly => serializer.serialize_str("weekly"),
            Self::Biweekly => serializer.serialize_str("biweekly"),
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::Quarterly => serializer.serialize_str("quarterly"),
            Self::Semiannually => serializer.serialize_str("semiannually"),
            Self::Annually => serializer.serialize_str("annually"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationReportRequestFrequency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "one-time" => Ok(Self::OneTime),
            "daily" => Ok(Self::Daily),
            "weekly" => Ok(Self::Weekly),
            "biweekly" => Ok(Self::Biweekly),
            "monthly" => Ok(Self::Monthly),
            "quarterly" => Ok(Self::Quarterly),
            "semiannually" => Ok(Self::Semiannually),
            "annually" => Ok(Self::Annually),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationReportRequestFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneTime => write!(f, "one-time"),
            Self::Daily => write!(f, "daily"),
            Self::Weekly => write!(f, "weekly"),
            Self::Biweekly => write!(f, "biweekly"),
            Self::Monthly => write!(f, "monthly"),
            Self::Quarterly => write!(f, "quarterly"),
            Self::Semiannually => write!(f, "semiannually"),
            Self::Annually => write!(f, "annually"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
