pub use crate::prelude::*;

/// Preferred payment method used.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BillQueryRecord2PaymentMethod {
    Vcard,
    Ach,
    Check,
    Card,
    Managed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BillQueryRecord2PaymentMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Vcard => serializer.serialize_str("vcard"),
            Self::Ach => serializer.serialize_str("ach"),
            Self::Check => serializer.serialize_str("check"),
            Self::Card => serializer.serialize_str("card"),
            Self::Managed => serializer.serialize_str("managed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BillQueryRecord2PaymentMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "vcard" => Ok(Self::Vcard),
            "ach" => Ok(Self::Ach),
            "check" => Ok(Self::Check),
            "card" => Ok(Self::Card),
            "managed" => Ok(Self::Managed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BillQueryRecord2PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vcard => write!(f, "vcard"),
            Self::Ach => write!(f, "ach"),
            Self::Check => write!(f, "check"),
            Self::Card => write!(f, "card"),
            Self::Managed => write!(f, "managed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
