pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationStandardRequestFrequency {
    OneTime,
    Untilcancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationStandardRequestFrequency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneTime => serializer.serialize_str("one-time"),
            Self::Untilcancelled => serializer.serialize_str("untilcancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationStandardRequestFrequency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "one-time" => Ok(Self::OneTime),
            "untilcancelled" => Ok(Self::Untilcancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationStandardRequestFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneTime => write!(f, "one-time"),
            Self::Untilcancelled => write!(f, "untilcancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
