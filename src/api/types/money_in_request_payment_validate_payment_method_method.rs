pub use crate::prelude::*;

/// The card validation method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RequestPaymentValidatePaymentMethodMethod {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "cloud")]
    Cloud,
}
impl fmt::Display for RequestPaymentValidatePaymentMethodMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Card => "card",
            Self::Cloud => "cloud",
        };
        write!(f, "{}", s)
    }
}
