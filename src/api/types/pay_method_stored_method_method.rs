pub use crate::prelude::*;

/// Method to use for the transaction. Use either `card` or `ach`, depending on what kind of method was tokenized to use a saved payment method for this transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayMethodStoredMethodMethod {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "ach")]
    Ach,
}
impl fmt::Display for PayMethodStoredMethodMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Card => "card",
            Self::Ach => "ach",
        };
        write!(f, "{}", s)
    }
}
