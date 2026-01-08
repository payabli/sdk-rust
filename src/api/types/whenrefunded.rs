pub use crate::prelude::*;

/// Describes the business refund policy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Whenrefunded {
    #[serde(rename = "Exchange Only")]
    ExchangeOnly,
    #[serde(rename = "No Refund or Exchange")]
    NoRefundOrExchange,
    #[serde(rename = "More than 30 days")]
    MoreThan30Days,
    #[serde(rename = "30 Days or Less")]
    ThirtyDaysOrLess,
}
impl fmt::Display for Whenrefunded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ExchangeOnly => "Exchange Only",
            Self::NoRefundOrExchange => "No Refund or Exchange",
            Self::MoreThan30Days => "More than 30 days",
            Self::ThirtyDaysOrLess => "30 Days or Less",
        };
        write!(f, "{}", s)
    }
}
