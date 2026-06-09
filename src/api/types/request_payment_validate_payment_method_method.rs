pub use crate::prelude::*;

/// The card validation method.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestPaymentValidatePaymentMethodMethod {
    Card,
    Cloud,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RequestPaymentValidatePaymentMethodMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Card => serializer.serialize_str("card"),
            Self::Cloud => serializer.serialize_str("cloud"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RequestPaymentValidatePaymentMethodMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "card" => Ok(Self::Card),
            "cloud" => Ok(Self::Cloud),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RequestPaymentValidatePaymentMethodMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Card => write!(f, "card"),
            Self::Cloud => write!(f, "cloud"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
