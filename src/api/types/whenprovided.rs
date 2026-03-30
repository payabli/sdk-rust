pub use crate::prelude::*;

/// Describes when goods or services are provided, from time of transaction.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Whenprovided {
    ThirtyDaysOrLess,
    ThirtyOneTo60Days,
    SixtyDays,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Whenprovided {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ThirtyDaysOrLess => serializer.serialize_str("30 Days or Less"),
            Self::ThirtyOneTo60Days => serializer.serialize_str("31 to 60 Days"),
            Self::SixtyDays => serializer.serialize_str("60+ Days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Whenprovided {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "30 Days or Less" => Ok(Self::ThirtyDaysOrLess),
            "31 to 60 Days" => Ok(Self::ThirtyOneTo60Days),
            "60+ Days" => Ok(Self::SixtyDays),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Whenprovided {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThirtyDaysOrLess => write!(f, "30 Days or Less"),
            Self::ThirtyOneTo60Days => write!(f, "31 to 60 Days"),
            Self::SixtyDays => write!(f, "60+ Days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
