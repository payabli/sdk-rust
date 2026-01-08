pub use crate::prelude::*;

/// Payment method used for the transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransactionDetailRecordMethod {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "card")]
    Card,
}
impl fmt::Display for TransactionDetailRecordMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ach => "ach",
            Self::Card => "card",
        };
        write!(f, "{}", s)
    }
}
