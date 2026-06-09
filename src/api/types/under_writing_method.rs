pub use crate::prelude::*;

/// This field controls which method is used to handle risk orchestration.
///
/// - `automatic`: Sends the application through the automatic
/// underwriting workflow using the provided `policyId`.
/// - `manual`: Puts the application into the pending review status. An
/// analyst must manually change its final status to approved or
/// declined.
/// - `bypass`: The application won't go through Payabli's review, and
/// proceeds directly to boarding products and services.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnderWritingMethod {
    Automatic,
    Manual,
    Bypass,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UnderWritingMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Automatic => serializer.serialize_str("automatic"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::Bypass => serializer.serialize_str("bypass"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UnderWritingMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),
            "bypass" => Ok(Self::Bypass),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UnderWritingMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automatic => write!(f, "automatic"),
            Self::Manual => write!(f, "manual"),
            Self::Bypass => write!(f, "bypass"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
