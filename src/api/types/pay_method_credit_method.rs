pub use crate::prelude::*;

/// Method to use for the transaction. For transactions with a credit or debit card, or a tokenized card, use `card`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayMethodCreditMethod {
    #[serde(rename = "card")]
    Card,
}
impl fmt::Display for PayMethodCreditMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Card => "card",
        };
        write!(f, "{}", s)
    }
}
