pub use crate::prelude::*;

/// Frequency for operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Frequency {
    #[serde(rename = "onetime")]
    OneTime,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "every2weeks")]
    Every2Weeks,
    #[serde(rename = "every6months")]
    Every6Months,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "every3months")]
    Every3Months,
    #[serde(rename = "annually")]
    Annually,
}
impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OneTime => "onetime",
            Self::Weekly => "weekly",
            Self::Every2Weeks => "every2weeks",
            Self::Every6Months => "every6months",
            Self::Monthly => "monthly",
            Self::Every3Months => "every3months",
            Self::Annually => "annually",
        };
        write!(f, "{}", s)
    }
}
