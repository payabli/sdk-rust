pub use crate::prelude::*;

/// Specify the size of the custom payment button.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonElementSize {
    Sm,
    Md,
    Lg,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ButtonElementSize {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sm => serializer.serialize_str("sm"),
            Self::Md => serializer.serialize_str("md"),
            Self::Lg => serializer.serialize_str("lg"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ButtonElementSize {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sm" => Ok(Self::Sm),
            "md" => Ok(Self::Md),
            "lg" => Ok(Self::Lg),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ButtonElementSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sm => write!(f, "sm"),
            Self::Md => write!(f, "md"),
            Self::Lg => write!(f, "lg"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
