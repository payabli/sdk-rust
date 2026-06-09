pub use crate::prelude::*;

/// The status to set on the card. Not all transitions are valid: `Active` can change to `Inactive`, `Cancelled`, or `Expired`. `Inactive` can change to `Active`. `Expired` can change to `Active` (renews the card). `Cancelled` is terminal and can't be changed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardStatus {
    Active,
    Inactive,
    Cancelled,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CardStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("Active"),
            Self::Inactive => serializer.serialize_str("Inactive"),
            Self::Cancelled => serializer.serialize_str("Cancelled"),
            Self::Expired => serializer.serialize_str("Expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CardStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Active" => Ok(Self::Active),
            "Inactive" => Ok(Self::Inactive),
            "Cancelled" => Ok(Self::Cancelled),
            "Expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CardStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Inactive => write!(f, "Inactive"),
            Self::Cancelled => write!(f, "Cancelled"),
            Self::Expired => write!(f, "Expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
