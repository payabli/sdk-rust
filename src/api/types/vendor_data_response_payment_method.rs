pub use crate::prelude::*;

/// Preferred payment method for vendor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VendorDataResponsePaymentMethod {
    #[serde(rename = "vcard")]
    Vcard,
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "card")]
    Card,
}
impl fmt::Display for VendorDataResponsePaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vcard => "vcard",
            Self::Ach => "ach",
            Self::Check => "check",
            Self::Card => "card",
        };
        write!(f, "{}", s)
    }
}
