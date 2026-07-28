pub use crate::prelude::*;

/// A Pay In service the bank account is used for.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MoneyInService {
    Ach,
    Card,
    Cloud,
    Device,
    Wallet,
    Cash,
    Check,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MoneyInService {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ach => serializer.serialize_str("Ach"),
            Self::Card => serializer.serialize_str("Card"),
            Self::Cloud => serializer.serialize_str("Cloud"),
            Self::Device => serializer.serialize_str("Device"),
            Self::Wallet => serializer.serialize_str("Wallet"),
            Self::Cash => serializer.serialize_str("Cash"),
            Self::Check => serializer.serialize_str("Check"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MoneyInService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Ach" => Ok(Self::Ach),
            "Card" => Ok(Self::Card),
            "Cloud" => Ok(Self::Cloud),
            "Device" => Ok(Self::Device),
            "Wallet" => Ok(Self::Wallet),
            "Cash" => Ok(Self::Cash),
            "Check" => Ok(Self::Check),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MoneyInService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ach => write!(f, "Ach"),
            Self::Card => write!(f, "Card"),
            Self::Cloud => write!(f, "Cloud"),
            Self::Device => write!(f, "Device"),
            Self::Wallet => write!(f, "Wallet"),
            Self::Cash => write!(f, "Cash"),
            Self::Check => write!(f, "Check"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
