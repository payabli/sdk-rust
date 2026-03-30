pub use crate::prelude::*;

/// The status of a payment link.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentLinkStatus {
    Active,
    Expired,
    Canceled,
    Deleted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentLinkStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("Active"),
            Self::Expired => serializer.serialize_str("Expired"),
            Self::Canceled => serializer.serialize_str("Canceled"),
            Self::Deleted => serializer.serialize_str("Deleted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentLinkStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Active" => Ok(Self::Active),
            "Expired" => Ok(Self::Expired),
            "Canceled" => Ok(Self::Canceled),
            "Deleted" => Ok(Self::Deleted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentLinkStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Expired => write!(f, "Expired"),
            Self::Canceled => write!(f, "Canceled"),
            Self::Deleted => write!(f, "Deleted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
