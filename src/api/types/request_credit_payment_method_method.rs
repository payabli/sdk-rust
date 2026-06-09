pub use crate::prelude::*;

/// Method to use for the transaction. Must be ACH.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RequestCreditPaymentMethodMethod {
    #[serde(rename = "ach")]
    Ach,
}
impl fmt::Display for RequestCreditPaymentMethodMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ach => "ach",
        };
        write!(f, "{}", s)
    }
}
