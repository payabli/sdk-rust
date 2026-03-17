pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodCredit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardcvv: Option<Cardcvv>,
    pub cardexp: Cardexp,
    #[serde(rename = "cardHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<Cardholder>,
    pub cardnumber: Cardnumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardzip: Option<Cardzip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    /// Method to use for the transaction. For transactions with a credit or debit card, or a tokenized card, use `card`.
    pub method: String,
    #[serde(rename = "saveIfSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_if_success: Option<SaveIfSuccess>,
}
