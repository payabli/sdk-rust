pub use crate::prelude::*;

/// Region where payment processing occurs
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OdpSetupProcessingRegion {
    Us,
    Ca,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OdpSetupProcessingRegion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Us => serializer.serialize_str("US"),
            Self::Ca => serializer.serialize_str("CA"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OdpSetupProcessingRegion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "US" => Ok(Self::Us),
            "CA" => Ok(Self::Ca),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OdpSetupProcessingRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Us => write!(f, "US"),
            Self::Ca => write!(f, "CA"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
