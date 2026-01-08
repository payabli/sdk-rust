pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TokenizeCard {
    /// The type of payment method to tokenize. For cards, this is always `card`.
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardcvv: Option<Cardcvv>,
    pub cardexp: Cardexp,
    #[serde(rename = "cardHolder")]
    pub card_holder: Cardholder,
    pub cardnumber: Cardnumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardzip: Option<Cardzip>,
}
