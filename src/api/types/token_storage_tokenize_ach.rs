pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TokenizeAch {
    /// The type of payment method to tokenize. For ACH, this is always `ach`.
    pub method: String,
    #[serde(rename = "achAccount")]
    pub ach_account: Achaccount,
    #[serde(rename = "achAccountType")]
    pub ach_account_type: Achaccounttype,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    #[serde(rename = "achHolder")]
    pub ach_holder: AchHolder,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "achRouting")]
    pub ach_routing: Achrouting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}
