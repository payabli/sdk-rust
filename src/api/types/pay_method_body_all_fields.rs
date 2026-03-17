pub use crate::prelude::*;

/// Model for the PaymentMethod object, includes all method types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodBodyAllFields {
    /// Bank account number. This field is **required** when method = 'ach'.
    #[serde(rename = "achAccount")]
    pub ach_account: Achaccount,
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<Achaccounttype>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    #[serde(rename = "achHolder")]
    pub ach_holder: AchHolder,
    /// ABA/routing number of Bank account. This field is **required** when method = 'ach'.
    #[serde(rename = "achRouting")]
    pub ach_routing: Achrouting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardcvv: Option<Cardcvv>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardexp: Option<Cardexp>,
    #[serde(rename = "cardHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<Cardholder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardnumber: Option<Cardnumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardzip: Option<Cardzip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initator: Option<Initiator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Methodall>,
    #[serde(rename = "saveIfSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_if_success: Option<SaveIfSuccess>,
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<Storedmethodid>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}
