pub use crate::prelude::*;

/// Method to use for the transaction. For cash transactions, use `cash`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CashMethod {
    #[serde(rename = "cash")]
    Cash,
}
impl fmt::Display for CashMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cash => "cash",
        };
        write!(f, "{}", s)
    }
}
