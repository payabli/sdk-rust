pub use crate::prelude::*;

/// Type of bank account: Checking or Savings.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeAccount {
    Checking,
    Savings,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TypeAccount {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Checking => serializer.serialize_str("Checking"),
            Self::Savings => serializer.serialize_str("Savings"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TypeAccount {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Checking" => Ok(Self::Checking),
            "Savings" => Ok(Self::Savings),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TypeAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checking => write!(f, "Checking"),
            Self::Savings => write!(f, "Savings"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
