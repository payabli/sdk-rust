pub use crate::prelude::*;

/// Object describing the ACH payment method to use for transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestCreditPaymentMethod {
    #[serde(rename = "achAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account: Option<Achaccount>,
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<Achaccounttype>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    /// Bank account holder.
    #[serde(rename = "achHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder: Option<AchHolder>,
    #[serde(rename = "achRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<Achrouting>,
    /// Method to use for the transaction. Must be ACH.
    pub method: String,
}
