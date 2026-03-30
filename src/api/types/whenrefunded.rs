pub use crate::prelude::*;

/// Describes the business refund policy.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Whenrefunded {
    ExchangeOnly,
    NoRefundOrExchange,
    MoreThan30Days,
    ThirtyDaysOrLess,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Whenrefunded {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ExchangeOnly => serializer.serialize_str("Exchange Only"),
            Self::NoRefundOrExchange => serializer.serialize_str("No Refund or Exchange"),
            Self::MoreThan30Days => serializer.serialize_str("More than 30 days"),
            Self::ThirtyDaysOrLess => serializer.serialize_str("30 Days or Less"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Whenrefunded {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Exchange Only" => Ok(Self::ExchangeOnly),
            "No Refund or Exchange" => Ok(Self::NoRefundOrExchange),
            "More than 30 days" => Ok(Self::MoreThan30Days),
            "30 Days or Less" => Ok(Self::ThirtyDaysOrLess),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Whenrefunded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExchangeOnly => write!(f, "Exchange Only"),
            Self::NoRefundOrExchange => write!(f, "No Refund or Exchange"),
            Self::MoreThan30Days => write!(f, "More than 30 days"),
            Self::ThirtyDaysOrLess => write!(f, "30 Days or Less"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
