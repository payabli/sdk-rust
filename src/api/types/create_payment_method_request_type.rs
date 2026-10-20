pub use crate::prelude::*;

/// The payment method type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatePaymentMethodRequestType {
    #[serde(rename = "bank_account")]
    BankAccount,
}
impl fmt::Display for CreatePaymentMethodRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::BankAccount => "bank_account",
        };
        write!(f, "{}", s)
    }
}
