pub use crate::prelude::*;

/// Frequency for operation. The `firstofmonth`, `fifteenthofmonth`, and `endofmonth` values are only valid on subscriptions — they aren't accepted by other endpoints (such as invoice scheduling) that use this enum.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Frequency {
    OneTime,
    Weekly,
    Every2Weeks,
    Every6Months,
    Monthly,
    Every3Months,
    Annually,
    FirstOfMonth,
    FifteenthOfMonth,
    EndOfMonth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Frequency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneTime => serializer.serialize_str("onetime"),
            Self::Weekly => serializer.serialize_str("weekly"),
            Self::Every2Weeks => serializer.serialize_str("every2weeks"),
            Self::Every6Months => serializer.serialize_str("every6months"),
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::Every3Months => serializer.serialize_str("every3months"),
            Self::Annually => serializer.serialize_str("annually"),
            Self::FirstOfMonth => serializer.serialize_str("firstofmonth"),
            Self::FifteenthOfMonth => serializer.serialize_str("fifteenthofmonth"),
            Self::EndOfMonth => serializer.serialize_str("endofmonth"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Frequency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "onetime" => Ok(Self::OneTime),
            "weekly" => Ok(Self::Weekly),
            "every2weeks" => Ok(Self::Every2Weeks),
            "every6months" => Ok(Self::Every6Months),
            "monthly" => Ok(Self::Monthly),
            "every3months" => Ok(Self::Every3Months),
            "annually" => Ok(Self::Annually),
            "firstofmonth" => Ok(Self::FirstOfMonth),
            "fifteenthofmonth" => Ok(Self::FifteenthOfMonth),
            "endofmonth" => Ok(Self::EndOfMonth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneTime => write!(f, "onetime"),
            Self::Weekly => write!(f, "weekly"),
            Self::Every2Weeks => write!(f, "every2weeks"),
            Self::Every6Months => write!(f, "every6months"),
            Self::Monthly => write!(f, "monthly"),
            Self::Every3Months => write!(f, "every3months"),
            Self::Annually => write!(f, "annually"),
            Self::FirstOfMonth => write!(f, "firstofmonth"),
            Self::FifteenthOfMonth => write!(f, "fifteenthofmonth"),
            Self::EndOfMonth => write!(f, "endofmonth"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
