pub use crate::prelude::*;

/// Payment method used for the transaction
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransactionDetailRecordMethod {
    Ach,
    Card,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TransactionDetailRecordMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ach => serializer.serialize_str("ach"),
            Self::Card => serializer.serialize_str("card"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TransactionDetailRecordMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ach" => Ok(Self::Ach),
            "card" => Ok(Self::Card),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TransactionDetailRecordMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ach => write!(f, "ach"),
            Self::Card => write!(f, "card"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
