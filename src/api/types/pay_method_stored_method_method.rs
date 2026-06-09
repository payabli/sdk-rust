pub use crate::prelude::*;

/// Method to use for the transaction. Use `card`, `ach`, or `wallet` depending
/// on what kind of method was tokenized to use a saved payment method for
/// this transaction.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PayMethodStoredMethodMethod {
    Card,
    Ach,
    Wallet,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PayMethodStoredMethodMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Card => serializer.serialize_str("card"),
            Self::Ach => serializer.serialize_str("ach"),
            Self::Wallet => serializer.serialize_str("wallet"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PayMethodStoredMethodMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "card" => Ok(Self::Card),
            "ach" => Ok(Self::Ach),
            "wallet" => Ok(Self::Wallet),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PayMethodStoredMethodMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Card => write!(f, "card"),
            Self::Ach => write!(f, "ach"),
            Self::Wallet => write!(f, "wallet"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
