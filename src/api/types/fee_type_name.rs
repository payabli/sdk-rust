pub use crate::prelude::*;

/// Pricing model, serialized as a name by the List profiles endpoint.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeeTypeName {
    Flat,
    Icp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FeeTypeName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Flat => serializer.serialize_str("Flat"),
            Self::Icp => serializer.serialize_str("ICP"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FeeTypeName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Flat" => Ok(Self::Flat),
            "ICP" => Ok(Self::Icp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FeeTypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flat => write!(f, "Flat"),
            Self::Icp => write!(f, "ICP"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
