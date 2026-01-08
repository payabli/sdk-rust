pub use crate::prelude::*;

/// When goods and services are delivered.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Whendelivered {
    #[serde(rename = "0-7 Days")]
    Zero7Days,
    #[serde(rename = "8-14 Days")]
    Eight14Days,
    #[serde(rename = "15-30 Days")]
    Fifteen30Days,
    #[serde(rename = "Over 30 Days")]
    Over30Days,
}
impl fmt::Display for Whendelivered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Zero7Days => "0-7 Days",
            Self::Eight14Days => "8-14 Days",
            Self::Fifteen30Days => "15-30 Days",
            Self::Over30Days => "Over 30 Days",
        };
        write!(f, "{}", s)
    }
}
