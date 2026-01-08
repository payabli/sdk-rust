pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodAch {
    /// Bank account number. This field is **required** when method = 'ach'.
    #[serde(rename = "achAccount")]
    pub ach_account: Achaccount,
    /// Bank account type. This field is **required** when method = 'ach'.
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<Achaccounttype>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    #[serde(rename = "achHolder")]
    pub ach_holder: AchHolder,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    /// ABA/routing number of bank account. This field is **required** when method = 'ach'.
    #[serde(rename = "achRouting")]
    pub ach_routing: Achrouting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    pub method: String,
}
