pub use crate::prelude::*;

/// Preferred payment method used.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BillQueryRecord2PaymentMethod {
    #[serde(rename = "vcard")]
    Vcard,
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "managed")]
    Managed,
}
impl fmt::Display for BillQueryRecord2PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vcard => "vcard",
            Self::Ach => "ach",
            Self::Check => "check",
            Self::Card => "card",
            Self::Managed => "managed",
        };
        write!(f, "{}", s)
    }
}
