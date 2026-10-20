pub use crate::prelude::*;

/// How applications from this template are underwritten.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnderwritingMode {
    Automatic,
    Manual,
    Bypass,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UnderwritingMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Automatic => serializer.serialize_str("automatic"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::Bypass => serializer.serialize_str("bypass"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UnderwritingMode {
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

impl fmt::Display for UnderwritingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automatic => write!(f, "automatic"),
            Self::Manual => write!(f, "manual"),
            Self::Bypass => write!(f, "bypass"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
