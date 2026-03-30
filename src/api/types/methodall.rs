pub use crate::prelude::*;

/// Method to use for the transaction.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Methodall {
    Card,
    Ach,
    Cloud,
    Check,
    Cash,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Methodall {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Card => serializer.serialize_str("card"),
            Self::Ach => serializer.serialize_str("ach"),
            Self::Cloud => serializer.serialize_str("cloud"),
            Self::Check => serializer.serialize_str("check"),
            Self::Cash => serializer.serialize_str("cash"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Methodall {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "card" => Ok(Self::Card),
            "ach" => Ok(Self::Ach),
            "cloud" => Ok(Self::Cloud),
            "check" => Ok(Self::Check),
            "cash" => Ok(Self::Cash),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Methodall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Card => write!(f, "card"),
            Self::Ach => write!(f, "ach"),
            Self::Cloud => write!(f, "cloud"),
            Self::Check => write!(f, "check"),
            Self::Cash => write!(f, "cash"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
