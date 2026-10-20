pub use crate::prelude::*;

/// The role an address plays for a business.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddressType {
    Legal,
    Billing,
    Shipping,
    Remittance,
    Physical,
    Mailing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AddressType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Legal => serializer.serialize_str("legal"),
            Self::Billing => serializer.serialize_str("billing"),
            Self::Shipping => serializer.serialize_str("shipping"),
            Self::Remittance => serializer.serialize_str("remittance"),
            Self::Physical => serializer.serialize_str("physical"),
            Self::Mailing => serializer.serialize_str("mailing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "legal" => Ok(Self::Legal),
            "billing" => Ok(Self::Billing),
            "shipping" => Ok(Self::Shipping),
            "remittance" => Ok(Self::Remittance),
            "physical" => Ok(Self::Physical),
            "mailing" => Ok(Self::Mailing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Legal => write!(f, "legal"),
            Self::Billing => write!(f, "billing"),
            Self::Shipping => write!(f, "shipping"),
            Self::Remittance => write!(f, "remittance"),
            Self::Physical => write!(f, "physical"),
            Self::Mailing => write!(f, "mailing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
