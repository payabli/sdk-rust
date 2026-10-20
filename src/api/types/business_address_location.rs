pub use crate::prelude::*;

/// The physical use of an address.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BusinessAddressLocation {
    Office,
    Warehouse,
    Retail,
    Home,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BusinessAddressLocation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Office => serializer.serialize_str("office"),
            Self::Warehouse => serializer.serialize_str("warehouse"),
            Self::Retail => serializer.serialize_str("retail"),
            Self::Home => serializer.serialize_str("home"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BusinessAddressLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "office" => Ok(Self::Office),
            "warehouse" => Ok(Self::Warehouse),
            "retail" => Ok(Self::Retail),
            "home" => Ok(Self::Home),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BusinessAddressLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Office => write!(f, "office"),
            Self::Warehouse => write!(f, "warehouse"),
            Self::Retail => write!(f, "retail"),
            Self::Home => write!(f, "home"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
