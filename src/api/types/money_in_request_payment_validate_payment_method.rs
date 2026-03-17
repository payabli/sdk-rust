pub use crate::prelude::*;

/// Object describing payment method to use for validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestPaymentValidatePaymentMethod {
    pub method: RequestPaymentValidatePaymentMethodMethod,
    pub cardnumber: Cardnumber,
    pub cardexp: Cardexp,
    pub cardzip: Cardzip,
    #[serde(rename = "cardHolder")]
    pub card_holder: Cardholder,
}
