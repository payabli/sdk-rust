pub use crate::prelude::*;

/// When goods and services are delivered.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Whendelivered {
    Zero7Days,
    Eight14Days,
    Fifteen30Days,
    Over30Days,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Whendelivered {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Zero7Days => serializer.serialize_str("0-7 Days"),
            Self::Eight14Days => serializer.serialize_str("8-14 Days"),
            Self::Fifteen30Days => serializer.serialize_str("15-30 Days"),
            Self::Over30Days => serializer.serialize_str("Over 30 Days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Whendelivered {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0-7 Days" => Ok(Self::Zero7Days),
            "8-14 Days" => Ok(Self::Eight14Days),
            "15-30 Days" => Ok(Self::Fifteen30Days),
            "Over 30 Days" => Ok(Self::Over30Days),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Whendelivered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero7Days => write!(f, "0-7 Days"),
            Self::Eight14Days => write!(f, "8-14 Days"),
            Self::Fifteen30Days => write!(f, "15-30 Days"),
            Self::Over30Days => write!(f, "Over 30 Days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
