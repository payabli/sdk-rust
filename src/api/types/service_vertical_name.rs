pub use crate::prelude::*;

/// Billing vertical, serialized as a name by the List profiles endpoint.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServiceVerticalName {
    PayIn,
    PayOut,
    PayOps,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ServiceVerticalName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PayIn => serializer.serialize_str("PayIn"),
            Self::PayOut => serializer.serialize_str("PayOut"),
            Self::PayOps => serializer.serialize_str("PayOps"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ServiceVerticalName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PayIn" => Ok(Self::PayIn),
            "PayOut" => Ok(Self::PayOut),
            "PayOps" => Ok(Self::PayOps),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ServiceVerticalName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayIn => write!(f, "PayIn"),
            Self::PayOut => write!(f, "PayOut"),
            Self::PayOps => write!(f, "PayOps"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
