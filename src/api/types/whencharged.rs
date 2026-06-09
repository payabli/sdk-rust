pub use crate::prelude::*;

/// Describes when customers are charged for goods or services.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Whencharged {
    WhenServiceProvided,
    InAdvance,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Whencharged {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WhenServiceProvided => serializer.serialize_str("When Service Provided"),
            Self::InAdvance => serializer.serialize_str("In Advance"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Whencharged {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "When Service Provided" => Ok(Self::WhenServiceProvided),
            "In Advance" => Ok(Self::InAdvance),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Whencharged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WhenServiceProvided => write!(f, "When Service Provided"),
            Self::InAdvance => write!(f, "In Advance"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
