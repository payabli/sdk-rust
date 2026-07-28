pub use crate::prelude::*;

/// A Pay Out service the bank account is used for.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MoneyOutService {
    Ach,
    VCard,
    Managed,
    Check,
    Rtp,
    Wire,
    Ghost,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MoneyOutService {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ach => serializer.serialize_str("Ach"),
            Self::VCard => serializer.serialize_str("VCard"),
            Self::Managed => serializer.serialize_str("Managed"),
            Self::Check => serializer.serialize_str("Check"),
            Self::Rtp => serializer.serialize_str("Rtp"),
            Self::Wire => serializer.serialize_str("Wire"),
            Self::Ghost => serializer.serialize_str("Ghost"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MoneyOutService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Ach" => Ok(Self::Ach),
            "VCard" => Ok(Self::VCard),
            "Managed" => Ok(Self::Managed),
            "Check" => Ok(Self::Check),
            "Rtp" => Ok(Self::Rtp),
            "Wire" => Ok(Self::Wire),
            "Ghost" => Ok(Self::Ghost),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MoneyOutService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ach => write!(f, "Ach"),
            Self::VCard => write!(f, "VCard"),
            Self::Managed => write!(f, "Managed"),
            Self::Check => write!(f, "Check"),
            Self::Rtp => write!(f, "Rtp"),
            Self::Wire => write!(f, "Wire"),
            Self::Ghost => write!(f, "Ghost"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
