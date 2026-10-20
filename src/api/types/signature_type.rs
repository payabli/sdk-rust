pub use crate::prelude::*;

/// How the signer signed. Case-insensitive.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignatureType {
    Type,
    Draw,
    Checkbox,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SignatureType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Type => serializer.serialize_str("type"),
            Self::Draw => serializer.serialize_str("draw"),
            Self::Checkbox => serializer.serialize_str("checkbox"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SignatureType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "type" => Ok(Self::Type),
            "draw" => Ok(Self::Draw),
            "checkbox" => Ok(Self::Checkbox),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SignatureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "type"),
            Self::Draw => write!(f, "draw"),
            Self::Checkbox => write!(f, "checkbox"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
