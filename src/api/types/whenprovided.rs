pub use crate::prelude::*;

/// Describes when goods or services are provided, from time of transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Whenprovided {
    #[serde(rename = "30 Days or Less")]
    ThirtyDaysOrLess,
    #[serde(rename = "31 to 60 Days")]
    ThirtyOneTo60Days,
    #[serde(rename = "60+ Days")]
    SixtyDays,
}
impl fmt::Display for Whenprovided {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ThirtyDaysOrLess => "30 Days or Less",
            Self::ThirtyOneTo60Days => "31 to 60 Days",
            Self::SixtyDays => "60+ Days",
        };
        write!(f, "{}", s)
    }
}
